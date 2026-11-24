use crate::{
    dialogs::{save_epub, select_epub, select_translated_folder},
    epub::{DocBuilder, FormatPage},
    error::{Error, Result, TaskExt as _},
    message::{SendAsync as _, Update},
    screen::format::{Format, message::Message},
};
use gpui_kit::{AppContext as _, Context, ExternalPaths, Image, ImageFormat, Window};
use rbook::Epub;
use smol::{
    fs::{self, read_dir, read_to_string},
    stream::StreamExt,
};
use std::{
    io::Cursor,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

impl Update for Format {
    type Message = Message;

    fn update(&mut self, message: Message, window: &mut Window, cx: &mut Context<Self>) {
        match message {
            Message::SelectEpub => self.select_epub(window, cx),
            Message::DropEpub(paths) => self.drop_epub(&paths, window, cx),
            Message::SelectFolder => self.select_folder(window, cx),
            Message::DropFolder(paths) => self.drop_folder(&paths, window, cx),
            Message::TitleChanged(title) => self.metadata.title = title,
            Message::AuthorsChanged(authors) => self.metadata.authors = authors,
            Message::Build => self.build_epub(window, cx),
            Message::EpubLoaded(path, epub) => {
                if let Err(error) = self.set_epub(path, *epub, window, cx) {
                    error.prompt("Failed to open epub", window, cx);
                }
            }
            Message::FolderLoaded(source_folder, pages) => self.set_pages(source_folder, pages),
        }
    }
}

impl Format {
    fn select_epub(&self, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |this, cx| {
            if let Some(handle) = select_epub().await {
                let buffer = handle.read().await;
                let epub = cx
                    .background_spawn(async { Epub::read(Cursor::new(buffer)) })
                    .await?;
                this.send(
                    cx,
                    Message::EpubLoaded(handle.path().into(), Box::new(epub)),
                )?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open epub", window, cx);
    }

    fn drop_epub(&self, paths: &ExternalPaths, window: &mut Window, cx: &mut Context<Self>) {
        if paths.paths().len() != 1 {
            return;
        }
        if let Some(path) = paths.paths().first().cloned()
            && path.is_file()
            && path.extension().is_some_and(|p| p == "epub")
        {
            cx.spawn_in(window, async move |this, cx| {
                let buffer = fs::read(&path).await?;
                let epub = cx
                    .background_spawn(async { Epub::read(Cursor::new(buffer)) })
                    .await?;
                this.send(cx, Message::EpubLoaded(path, Box::new(epub)))?;
                Ok(())
            })
            .detach_and_prompt_err("Failed to load epub", window, cx);
        };
    }

    fn drop_folder(&self, paths: &ExternalPaths, window: &mut Window, cx: &mut Context<Self>) {
        if paths.paths().len() != 1 {
            return;
        }
        if let Some(path) = paths.paths().first().cloned()
            && path.is_dir()
        {
            cx.spawn(async move |this, cx| {
                let folder_path = path.clone();
                let pages = cx
                    .background_spawn(read_markdown_files(folder_path))
                    .await?;
                let file_name = path.file_name().map_or_default(|s| s.to_string_lossy());
                this.send(cx, Message::FolderLoaded(file_name.into_owned(), pages))?;
                Ok(())
            })
            .detach_and_prompt_err("Failed to load markdown", window, cx);
        };
    }

    fn set_epub(
        &mut self,
        path: impl Into<PathBuf>,
        epub: Epub,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let path: PathBuf = path.into();

        self.cover = epub.manifest().cover_image().and_then(|e| {
            let mime_type = mime_guess::from_ext(e.href().extension()?).first_raw()?;
            let format = ImageFormat::from_mime_type(mime_type)?;
            Some(Arc::new(Image::from_bytes(format, e.read_bytes().ok()?)))
        });
        self.set_title(&path, window, cx);
        self.set_authors(&epub, window, cx);

        self.epub_path = Some(path);
        self.epub = Some(epub);
        Ok(())
    }

    fn set_title(&mut self, path: &Path, window: &mut Window, cx: &mut Context<Self>) {
        let title = path
            .file_prefix()
            .map_or_default(|e| e.to_string_lossy().to_string());
        self.inputs
            .title
            .update(cx, |this, cx| this.set_value(title.clone(), window, cx));
        self.metadata.title = title;
    }

    fn set_authors(&mut self, epub: &Epub, window: &mut Window, cx: &mut Context<Self>) {
        let authors = epub
            .metadata()
            .creators()
            .map(|e| e.value())
            .collect::<Vec<_>>()
            .join(" & ");
        self.inputs
            .authors
            .update(cx, |this, cx| this.set_value(authors.clone(), window, cx));
        self.metadata.authors = authors;
    }

    fn select_folder(&self, window: &mut Window, cx: &mut Context<Self>) {
        let dir = self
            .epub_path
            .as_ref()
            .and_then(|p| Some(p.parent()?.into()));
        cx.spawn(async |this, cx| {
            if let Some(handle) = select_translated_folder(dir).await {
                let folder_path = handle.path().to_path_buf();
                let pages = cx
                    .background_spawn(read_markdown_files(folder_path))
                    .await?;
                this.send(cx, Message::FolderLoaded(handle.file_name(), pages))?;
            }
            Ok(())
        })
        .detach_and_prompt_err("Failed to open source files", window, cx);
    }

    fn set_pages(&mut self, source_folder: impl Into<String>, pages: Vec<(PathBuf, String)>) {
        self.pages = pages.into_iter().map(FormatPage::from).collect();
        self.source_folder = Some(source_folder.into());
    }

    fn build_epub(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let builder = self.get_build_content();
        self.inputs.clear(window, cx);

        cx.spawn(async |_, cx| {
            let (content, file_name) = cx.background_spawn(async move { builder?.build() }).await?;
            if let Some(handle) = save_epub(file_name).await {
                handle.write(&content).await?
            }

            Ok(())
        })
        .detach_and_prompt_err("Failed to build epub", window, cx);
    }

    pub fn get_build_content(&mut self) -> Result<DocBuilder> {
        let epub = mem::take(&mut self.epub).ok_or(Error::BuildError("Epub not found"))?;
        let pages = mem::take(&mut self.pages);
        let name = self
            .epub_path
            .take()
            .and_then(|e| Some(e.file_name()?.to_string_lossy().to_string()))
            .unwrap_or_default();
        let metadata = mem::take(&mut self.metadata);

        self.source_folder = None;
        self.cover = None;

        Ok(DocBuilder::new(epub, name, pages, metadata))
    }
}

async fn read_markdown_files(folder_path: impl AsRef<Path>) -> Result<Vec<(PathBuf, String)>> {
    let mut paths = read_dir(&folder_path).await?;
    let mut pages = Vec::new();
    while let Some(entry) = paths.next().await {
        let path = entry?.path();
        if path.is_file() && path.extension().is_some_and(|e| e == "md") {
            let content = read_to_string(&path).await?;
            pages.push((path, content));
        }
    }
    Ok(pages)
}

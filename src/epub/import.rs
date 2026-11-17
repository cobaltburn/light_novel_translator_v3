use crate::{
    chapter::Chapter,
    epub::html::{html_to_markdown, html_to_marked_markdown, toc_to_markdown},
    error::Result,
    page::{Page, TOC_PAGE_PREFIX},
    text::{has_translatable_text, markdown_sections},
};
use rbook::Epub;
use std::{io::Cursor, iter, path::PathBuf};

pub fn get_pages(buffer: Vec<u8>) -> Result<Vec<Page>> {
    let epub = Epub::read(Cursor::new(buffer))?;
    let path = PathBuf::from(TOC_PAGE_PREFIX).with_extension("xhtml");
    let toc = Page::new(path, vec![toc_to_markdown(epub.toc())?]);

    let pages: Result<Vec<_>> = epub
        .spine()
        .iter()
        .filter_map(|e| e.manifest_entry())
        .map(|entry| {
            let html = entry.read_str()?;
            let path = entry.href().decode().as_ref().into();
            Ok((path, html_to_marked_markdown(&html)?))
        })
        .map(|result| {
            result.map(|(path, markdown)| {
                let sections = if has_translatable_text(&markdown) {
                    markdown_sections(&markdown)
                } else {
                    Vec::new()
                };
                Page::new(path, sections)
            })
        })
        .collect();

    let pages: Vec<_> = iter::once(toc)
        .chain(pages?)
        .filter(|p| !p.sections.is_empty())
        .collect();

    Ok(pages)
}

pub async fn get_chapters(buffer: Vec<u8>) -> Result<Vec<Chapter>> {
    let epub = Epub::read(Cursor::new(buffer))?;

    let path = PathBuf::from(TOC_PAGE_PREFIX).with_extension("xhtml");
    let toc = Chapter::new(path, toc_to_markdown(epub.toc())?);

    let chapters: Vec<_> = epub
        .spine()
        .iter()
        .filter_map(|e| e.manifest_entry())
        .map(|entry| {
            let html = entry.read_str()?;
            let path = entry.href().decode().as_ref().into();
            Ok(Chapter::new(path, html_to_markdown(&html)?))
        })
        .filter(|m| m.as_ref().map_or(true, |t| has_translatable_text(&t.text)))
        .collect::<Result<_>>()?;

    Ok(iter::once(toc).chain(chapters).collect())
}

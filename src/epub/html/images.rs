use crate::{
    epub::{
        html::{
            resource_links::{IMAGES, update_tag_path},
            strip::strip_syosetu_tags,
            tags::{HEAD_TAG, IMG_TAG, SRC, XLINK, is_image},
            text::TextRun,
        },
        resources::PageLinks,
    },
    error::Result,
    text::image_marker,
};
use quick_xml::{
    Reader, Writer, XmlVersion,
    events::{BytesStart, BytesText, Event},
};
use std::io::Cursor;

pub fn insert_image_markers(html: &str) -> Result<String> {
    let mut reader = Reader::from_str(html);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let mut count = 0;

    loop {
        match reader.read_event()? {
            Event::Start(tag) if tag.name().as_ref() == HEAD_TAG => {
                reader.read_to_end(tag.name())?;
            }
            Event::Empty(tag) if is_image(&tag) => {
                writer
                    .create_element("p")
                    .write_text_content(BytesText::new(&image_marker(count)))?;
                count += 1;
            }
            Event::Eof => break,
            e => writer.write_event(e)?,
        }
    }

    Ok(String::from_utf8(writer.into_inner().into_inner())?)
}

pub fn image_anchors(
    html: &str,
    links: PageLinks<'_>,
) -> Result<Vec<(BytesStart<'static>, Option<String>)>> {
    let html = strip_syosetu_tags(html)?;
    let mut reader = Reader::from_str(&html);
    let mut anchors = vec![];
    let mut pending = vec![];
    let mut run = TextRun::default();

    loop {
        let event = reader.read_event()?;
        if run.push(&event)? {
            continue;
        }
        if let Some(text) = run.finish()
            && !pending.is_empty()
        {
            anchors.extend(pending.drain(..).map(|tag| (tag, Some(text.clone()))));
        }

        match event {
            Event::Start(tag) if tag.name().as_ref() == HEAD_TAG => {
                reader.read_to_end(tag.name())?;
            }
            Event::Empty(tag) if is_image(&tag) => {
                pending.push(html_image(tag, links)?);
            }
            Event::Eof => break,
            _ => (),
        }
    }

    anchors.extend(pending.into_iter().map(|tag| (tag, None)));
    Ok(anchors)
}

/// The `<img>` to write for an image of the source page, pointing at its output path.
///
/// An svg `<image>` only renders inside its `<svg>`, and its `xlink` prefix is
/// undeclared outside it, so it becomes an `<img>` of the same picture.
fn html_image(tag: BytesStart<'_>, links: PageLinks<'_>) -> Result<BytesStart<'static>> {
    if tag.name().as_ref() == IMG_TAG {
        return update_tag_path(tag, links, IMAGES, SRC);
    }
    let Some(link) = tag.try_get_attribute(XLINK)? else {
        return Ok(tag.into_owned());
    };

    let link = link.normalized_value(XmlVersion::Implicit1_0)?;
    let src = links.resolve(&link, IMAGES);
    Ok(BytesStart::new(IMG_TAG)
        .with_attributes([(SRC, src.as_str()), ("alt", "")])
        .into_owned())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::epub::resources::ResourcePaths;

    #[test]
    fn writes_svg_images_as_img() {
        let paths = ResourcePaths::default();
        let links = PageLinks {
            page: "/OEBPS/text/p-001.xhtml",
            paths: &paths,
        };
        let html = r#"<html xmlns="http://www.w3.org/1999/xhtml" xmlns:xlink="http://www.w3.org/1999/xlink">
<head><title>t</title></head>
<body>
<svg viewBox="0 0 1200 1700"><image width="1200" height="1700" xlink:href="../image/i-001.jpg"/></svg>
<p><img class="gaiji" src="../image/g-001.png"/></p>
<p>Text</p>
</body>
</html>"#;

        let anchors = image_anchors(html, links).unwrap();
        let tags: Vec<_> = anchors
            .iter()
            .map(|(tag, _)| -> &str { tag.as_ref() })
            .collect();
        assert_eq!(
            tags,
            [
                r#"img src="../Images/i-001.jpg" alt="""#,
                r#"img class="gaiji" src="../Images/g-001.png""#,
            ]
        );
    }
}

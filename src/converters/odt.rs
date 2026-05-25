use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub fn convert_odt(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    let mut content_xml = String::new();
    {
        let mut file = archive.by_name("content.xml").map_err(|e| e.to_string())?;
        file.read_to_string(&mut content_xml).map_err(|e| e.to_string())?;
    }

    let mut reader = Reader::from_str(&content_xml);
    reader.trim_text(true);

    let mut markdown = String::new();
    let mut buf = Vec::new();
    let mut current_text = String::new();
    let mut heading_level: Option<usize> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"text:h" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"text:outline-level" {
                                heading_level = Some(String::from_utf8_lossy(&attr.value).parse().unwrap_or(1));
                            }
                        }
                        current_text.clear();
                    }
                    b"text:p" => {
                        current_text.clear();
                        heading_level = None;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"text:h" => {
                        if let Some(level) = heading_level {
                            markdown.push_str(&"#".repeat(level));
                            markdown.push(' ');
                        }
                        markdown.push_str(&current_text);
                        markdown.push_str("\n\n");
                    }
                    b"text:p" => {
                        if !current_text.is_empty() {
                            markdown.push_str(&current_text);
                            markdown.push_str("\n\n");
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) => {
                current_text.push_str(&e.unescape().unwrap_or_default());
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        buf.clear();
    }

    Ok(markdown.trim().to_string())
}

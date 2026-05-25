use std::fs::File;
use std::io::Read;
use zip::ZipArchive;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub fn convert_docx(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    
    let mut document_xml = String::new();
    {
        let mut file = archive.by_name("word/document.xml").map_err(|e| e.to_string())?;
        file.read_to_string(&mut document_xml).map_err(|e| e.to_string())?;
    }

    let mut reader = Reader::from_str(&document_xml);
    reader.trim_text(true);

    let mut markdown = String::new();
    let mut buf = Vec::new();
    let mut current_text = String::new();
    let mut is_heading = false;
    let mut heading_level = 0;
    let mut in_table = false;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"w:tbl" => {
                        in_table = true;
                        table_rows.clear();
                    }
                    b"w:tr" => {
                        current_row.clear();
                    }
                    b"w:tc" => {
                        current_text.clear();
                    }
                    b"w:p" => {
                        if !in_table {
                            current_text.clear();
                            is_heading = false;
                            heading_level = 0;
                        }
                    }
                    b"w:pStyle" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"w:val" {
                                let val = String::from_utf8_lossy(&attr.value);
                                if val.contains("Heading") || val.parse::<usize>().is_ok() {
                                    is_heading = true;
                                    heading_level = val.chars().find(|c| c.is_digit(10))
                                        .and_then(|c| c.to_digit(10))
                                        .unwrap_or(1) as usize;
                                } else if val == "a3" {
                                    is_heading = true;
                                    heading_level = 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"w:p" => {
                        if !in_table && !current_text.is_empty() {
                            if is_heading {
                                markdown.push_str(&"#".repeat(heading_level));
                                markdown.push(' ');
                            }
                            markdown.push_str(&current_text);
                            markdown.push_str("\n\n");
                        }
                    }
                    b"w:tc" => {
                        current_row.push(current_text.clone());
                    }
                    b"w:tr" => {
                        table_rows.push(current_row.clone());
                    }
                    b"w:tbl" => {
                        in_table = false;
                        if !table_rows.is_empty() {
                            for (i, row) in table_rows.iter().enumerate() {
                                markdown.push_str("| ");
                                markdown.push_str(&row.join(" | "));
                                markdown.push_str(" |\n");
                                if i == 0 {
                                    markdown.push_str("| ");
                                    markdown.push_str(&vec!["---"; row.len()].join(" | "));
                                    markdown.push_str(" |\n");
                                }
                            }
                            markdown.push_str("\n");
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

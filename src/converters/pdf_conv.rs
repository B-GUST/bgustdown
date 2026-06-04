use lopdf::{Document, Object};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct TextBlock {
    text: String,
    font_size: f32,
}

pub fn convert_pdf(path: &str) -> Result<String, String> {
    let doc = Document::load(path).map_err(|e| e.to_string())?;
    
    let mut blocks = Vec::new();
    let mut font_sizes: HashMap<i32, i32> = HashMap::new();

    for (_page_num, page_id) in doc.get_pages() {
        let content_data = doc.get_page_content(page_id).unwrap_or_default();
        if let Ok(content) = lopdf::content::Content::decode(&content_data) {
            let mut current_font_size = 12.0;
            let mut current_text = String::new();

            for operation in content.operations {
                match operation.operator.as_str() {
                    "Tf" => {
                        if let Some(Object::Real(size)) = operation.operands.get(1) {
                            current_font_size = *size;
                        } else if let Some(Object::Integer(size)) = operation.operands.get(1) {
                            current_font_size = *size as f32;
                        }
                    }
                    "Tj" => {
                        if let Some(Object::String(bytes, _)) = operation.operands.get(0) {
                            current_text.push_str(&String::from_utf8_lossy(bytes));
                        }
                    }
                    "TJ" => {
                        if let Some(Object::Array(arr)) = operation.operands.get(0) {
                            for obj in arr {
                                match obj {
                                    Object::String(bytes, _) => {
                                        current_text.push_str(&String::from_utf8_lossy(bytes));
                                    }
                                    Object::Real(num) if *num < -50.0 => current_text.push(' '),
                                    Object::Integer(num) if *num < -50 => current_text.push(' '),
                                    _ => {}
                                }
                            }
                        }
                    }
                    "ET" | "Td" | "TD" | "Tm" | "T*" => {
                        if !current_text.trim().is_empty() {
                            blocks.push(TextBlock {
                                text: current_text.trim().to_string(),
                                font_size: current_font_size,
                            });
                            *font_sizes.entry((current_font_size * 10.0) as i32).or_insert(0) += 1;
                            current_text.clear();
                        }
                    }
                    _ => {}
                }
            }
            if !current_text.trim().is_empty() {
                blocks.push(TextBlock {
                    text: current_text.trim().to_string(),
                    font_size: current_font_size,
                });
                *font_sizes.entry((current_font_size * 10.0) as i32).or_insert(0) += 1;
            }
        }
    }

    if blocks.len() < 5 || blocks.iter().map(|b| b.text.len()).sum::<usize>() < 100 {
        // Fallback a bgustreadimg OCR (Surya ONNX)
        let _ = bgustreadimg::ocr::OcrEngine::new("../bgustreadimg/models/surya-onnx/model.onnx");
        return Ok("[Sugerencia: PDF escaneado detectado. Se procesará vía bgustreadimg (Surya OCR)]".to_string());
    }

    // Calcular el font size más común (texto base / párrafo)
    let mut base_font_size = 12.0;
    let mut max_count = 0;
    for (size, count) in font_sizes {
        if count > max_count {
            max_count = count;
            base_font_size = size as f32 / 10.0;
        }
    }

    let mut markdown = String::new();
    for block in blocks {
        let size_diff = block.font_size - base_font_size;
        
        let prefix = if size_diff >= 8.0 {
            "# "
        } else if size_diff >= 4.0 {
            "## "
        } else if size_diff >= 1.5 {
            "### "
        } else {
            ""
        };
        
        markdown.push_str(prefix);
        markdown.push_str(&block.text);
        markdown.push_str("\n\n");
    }

    Ok(markdown.trim().to_string())
}

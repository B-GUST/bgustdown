use pdf_extract::extract_text;

pub fn convert_pdf(path: &str) -> Result<String, String> {
    // 1. Intentar extracción de texto digital rápida
    let text = extract_text(path).map_err(|e| e.to_string())?;

    // 2. Heurística de detección de "PDF escaneado"
    // Si el texto extraído es muy corto (< 10 caracteres por página aprox) 
    // o está vacío, asumimos que necesita OCR.
    if text.trim().len() < 50 {
        return Ok(format!(
            "{}\n\n[Sugerencia: Se detectó un PDF posiblemente escaneado. bgustdown integrará OCR local en el siguiente commit de la Fase 4]", 
            text.trim()
        ));
    }

    // 3. Formateo a Markdown
    let mut markdown = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            markdown.push_str(trimmed);
            markdown.push_str("\n\n");
        }
    }

    Ok(markdown.trim().to_string())
}

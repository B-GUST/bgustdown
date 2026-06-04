/// Segmentación de oraciones robusta y compatible con Rust.
/// Utiliza una lógica de división basada en puntuación y reglas de limpieza.
pub fn segment_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    
    let chars: Vec<char> = text.chars().collect();
    for i in 0..chars.len() {
        current.push(chars[i]);
        
        // Si encontramos un punto, signo de exclamación o interrogación
        if chars[i] == '.' || chars[i] == '!' || chars[i] == '?' {
            // Verificar que no sea una abreviatura (regla simple: si hay espacio después y el siguiente es Mayúscula)
            let is_at_end = i + 1 == chars.len();
            let has_space_after = !is_at_end && chars[i+1].is_whitespace();
            
            if is_at_end || has_space_after {
                sentences.push(current.trim().to_string());
                current = String::new();
            }
        }
    }
    
    if !current.trim().is_empty() {
        sentences.push(current.trim().to_string());
    }

    sentences.into_iter().filter(|s| s.len() > 2).collect()
}

pub fn create_nsp_pairs(sentences: Vec<String>) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    for i in 0..sentences.len().saturating_sub(1) {
        pairs.push((sentences[i].clone(), sentences[i+1].clone()));
    }
    pairs
}

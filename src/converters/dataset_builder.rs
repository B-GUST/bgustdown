use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufWriter};
use regex::Regex;

#[derive(Serialize, Deserialize, Clone)]
pub struct TrainingExample {
    pub text: String,
    pub metadata: TrainingMetadata,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TrainingMetadata {
    pub source: String,
    pub domain: String,
    pub layout_order: usize,
}

pub struct DatasetCleaner;

impl DatasetCleaner {
    /// Elimina ruido común de documentos convertidos:
    /// - Headers/Footers repetitivos (ej. "Página 1 de 10")
    /// - Múltiples saltos de línea y espacios
    /// - Artefactos de conversión XML/HTML
    pub fn clean(text: &str) -> String {
        let mut cleaned = text.to_string();

        // 1. Normalizar saltos de línea
        let re_newlines = Regex::new(r"\n{3,}").unwrap();
        cleaned = re_newlines.replace_all(&cleaned, "\n\n").to_string();

        // 2. Eliminar numeración de página común
        let re_pages = Regex::new(r"(?i)p[áa]gina\s+\d+(\s+de\s+\d+)?").unwrap();
        cleaned = re_pages.replace_all(&cleaned, "").to_string();

        // 3. Eliminar múltiples espacios
        let re_spaces = Regex::new(r"[ \t]{2,}").unwrap();
        cleaned = re_spaces.replace_all(&cleaned, " ").to_string();

        cleaned.trim().to_string()
    }
}

pub fn export_to_jsonl(examples: Vec<TrainingExample>, output_path: &str) -> Result<(), String> {
    let file = File::create(output_path).map_err(|e| e.to_string())?;
    let mut writer = BufWriter::new(file);

    for example in examples {
        let json = serde_json::to_string(&example).map_err(|e| e.to_string())?;
        writer.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        writer.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

use napi_derive::napi;
use napi::{Error, Status, Result};
mod converters;
mod text_utils;

use converters::docx::convert_docx;
use converters::odt::convert_odt;
use converters::excel::convert_excel;
use converters::csv_conv::convert_csv;
use converters::pdf_conv::convert_pdf;
use std::path::Path;

#[napi]
pub async fn convert_file(path: String) -> Result<String> {
    let path_obj = Path::new(&path);
    let extension = path_obj.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "docx" => convert_docx(&path).map_err(|e| Error::new(Status::GenericFailure, e)),
        "odt" => convert_odt(&path).map_err(|e| Error::new(Status::GenericFailure, e)),
        "xlsx" | "xls" | "xlsm" | "xlsb" | "ods" => convert_excel(&path).map_err(|e| Error::new(Status::GenericFailure, e)),
        "csv" => convert_csv(&path).map_err(|e| Error::new(Status::GenericFailure, e)),
        "pdf" => convert_pdf(&path).map_err(|e| Error::new(Status::GenericFailure, e)),
        _ => Err(Error::new(Status::InvalidArg, format!("Unsupported file extension: {}", extension))),
    }
}

#[napi]
pub struct Bgustdown {}

#[napi]
impl Bgustdown {
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    #[napi]
    pub async fn convert(&self, path: String) -> Result<String> {
        let raw_markdown = convert_file(path).await?;
        // Aplicar limpieza semántica por defecto
        Ok(converters::dataset_builder::DatasetCleaner::clean(&raw_markdown))
    }

    #[napi]
    pub fn prepare_training_data(&self, text: String, _source: String, _domain: String) -> Result<Vec<String>> {
        let cleaned = converters::dataset_builder::DatasetCleaner::clean(&text);
        let sentences = text_utils::segment_sentences(&cleaned);
        Ok(sentences)
    }
}

#[napi]
pub fn convert_text(input: String) -> String {
    format!("Converted: {}", input)
}

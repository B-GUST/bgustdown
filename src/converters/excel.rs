use arrow::array::{ArrayRef, StringArray};
use arrow::record_batch::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use calamine::{open_workbook_auto, Reader, Sheets};
use std::sync::Arc;

pub fn convert_excel(path: &str) -> Result<String, String> {
    let mut workbook: Sheets<_> = open_workbook_auto(path).map_err(|e| e.to_string())?;
    let sheet_names = workbook.sheet_names().to_vec();
    
    let mut markdown = String::new();

    for sheet_name in sheet_names {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            markdown.push_str(&format!("## Sheet: {}\n\n", sheet_name));
            
            // Convert range to Arrow RecordBatch for optimized processing
            let width = range.width();
            let height = range.height();
            let mut columns: Vec<Vec<String>> = vec![vec![String::new(); height]; width];
            
            for (r, row) in range.rows().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    columns[c][r] = cell.to_string();
                }
            }

            let fields: Vec<Field> = (0..width)
                .map(|i| Field::new(format!("col_{}", i), DataType::Utf8, true))
                .collect();
            let schema = Arc::new(Schema::new(fields));

            let arrays: Vec<ArrayRef> = columns
                .into_iter()
                .map(|col| Arc::new(StringArray::from(col)) as ArrayRef)
                .collect();

            let batch = RecordBatch::try_new(schema, arrays).map_err(|e| e.to_string())?;

            // Generate Markdown Table from Arrow Batch
            for r in 0..batch.num_rows() {
                markdown.push_str("| ");
                for c in 0..batch.num_columns() {
                    let column = batch.column(c).as_any().downcast_ref::<StringArray>().unwrap();
                    markdown.push_str(column.value(r));
                    markdown.push_str(" | ");
                }
                markdown.push_str("\n");
                
                if r == 0 {
                    markdown.push_str("| ");
                    for _ in 0..batch.num_columns() {
                        markdown.push_str(" --- | ");
                    }
                    markdown.push_str("\n");
                }
            }
            markdown.push_str("\n");
        }
    }

    Ok(markdown.trim().to_string())
}

use arrow::array::{ArrayRef, StringArray};
use arrow::record_batch::RecordBatch;
use arrow_schema::{DataType, Field, Schema};
use csv::ReaderBuilder;
use std::sync::Arc;

pub fn convert_csv(path: &str) -> Result<String, String> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)
        .map_err(|e| e.to_string())?;

    let mut rows: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result.map_err(|e| e.to_string())?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        rows.push(row);
    }

    if rows.is_empty() {
        return Ok(String::new());
    }

    let width = rows[0].len();
    let mut columns: Vec<Vec<String>> = vec![vec![String::new(); rows.len()]; width];

    for (r, row) in rows.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if c < width {
                columns[c][r] = val.clone();
            }
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
    
    let mut markdown = String::new();
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

    Ok(markdown.trim().to_string())
}

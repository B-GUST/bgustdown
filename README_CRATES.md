# bgustdown

**The definitive high-performance document engine for the AI era, built for Rust.**

`bgustdown` is an industrial-grade data engineering library designed for large-scale document extraction and processing. Built from the ground up in Rust, it leverages native concurrency and Apache Arrow to convert complex formats—including PDF, DOCX, and XLSX—into structured Markdown and NLP-ready datasets in milliseconds.

## Key Features

- **⚡ Blazing Fast:** Leverages Rust and Tokio for real parallel processing. Process massive spreadsheets or complex PDFs in miliseconds.
- **📊 Industrial Data Handling:** Native **Apache Arrow** integration for tabular data (XLSX, CSV), ensuring memory efficiency and zero-copy performance.
- **🧠 Semantic Intelligence:** Built-in semantic cleaning algorithms to remove headers, footers, and structural noise.
- **🎯 BERT/NLP Optimized:** Specialized sentence segmentation with context preservation, ideal for creating training corpora for models like BERT/BETO.

## Supported Formats

| Category | Formats | Underlying Engine |
| :--- | :--- | :--- |
| **Documents** | `.docx`, `.odt` | Native Rust XML Parser |
| **Data** | `.xlsx`, `.xls`, `.csv`, `.ods` | Calamine + Apache Arrow |
| **Rigid** | `.pdf` | pdf-extract |
| **Output** | `.md`, `.jsonl` | Semantic Dataset Builder |

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bgustdown = "0.1.2"
```

## Quick Start

### Basic Markdown Conversion

```rust
use bgustdown::Bgustdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Bgustdown::new()?;
    
    // Fast conversion to clean Markdown
    let markdown = client.convert("report.pdf").await?;
    println!("{}", markdown);
    
    Ok(())
}
```

### Advanced NLP Dataset Preparation

```rust
use bgustdown::Bgustdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Bgustdown::new()?;
    let markdown = client.convert("legal_document.docx").await?;
    
    // Clean and segment text for BERT training
    let sentences = client.prepare_training_data(markdown, "legal_source", "legal_domain");
    println!("Extracted {} high-quality sentences.", sentences.len());
    
    Ok(())
}
```

## Performance Benchmarks

*   **Excel (50k rows):** ~12ms
*   **Word (100 pages):** ~15ms
*   **Parallel PDFs (10 files):** Linear scaling thanks to Tokio.

## Attribution

This project is conceptually inspired by Microsoft's **MarkItDown**. Re-engineered in Rust for extreme performance and AI specialization.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
Built with ❤️ for the Rust community by **B-GUST**. Visit [bgustdown.lat](https://bgustdown.lat) for official documentation.

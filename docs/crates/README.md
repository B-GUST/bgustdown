# bgustdown

**The definitive high-performance document engine for the AI era, built for Rust.**

`bgustdown` is an industrial-grade data engineering library designed for large-scale document extraction and processing. Built from the ground up in Rust, it leverages native concurrency and Apache Arrow to convert complex formats—including PDF, DOCX, and XLSX—into structured Markdown and NLP-ready datasets in milliseconds.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bgustdown = "0.1.3"
```

## What's New in v0.1.3

- **Native Rust Linkage (`rlib`):** The crate is now distributed as a standard Rust static library (`rlib`), allowing direct linking and native usage in standard Rust projects without relying on Node.js/NAPI.
- **Isolated Workspace Bounds:** Cleaned up project boundaries to allow standalone compilation (`cargo build`) outside of monorepo workspace restrictions.
- **Improved Crate Structure:** Standardized the distribution structure specifically for Cargo/crates.io.

## Quick Start

```rust
use bgustdown::Bgustdown;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Bgustdown::new()?;
    let markdown = client.convert("report.pdf").await?;
    println!("{}", markdown);
    Ok(())
}
```

## License
MIT License.

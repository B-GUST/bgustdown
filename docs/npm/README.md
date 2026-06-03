# bgustdown (Node.js/NPM)

**The definitive high-performance document engine for the AI era.**

`bgustdown` is a native Node.js library written in **Rust**, designed to convert complex documents into clean Markdown and NLP-ready datasets with extreme speed.

## Key Features

- **⚡ Blazing Fast:** Powered by a native Rust engine for real parallel processing.
- **📊 Industrial Data Handling:** Native **Apache Arrow** integration for tabular data (XLSX, CSV), ensuring memory efficiency.
- **🧠 Semantic Intelligence:** Built-in semantic cleaning to remove noise like page numbers, headers, and footers.
- **🎯 NLP Optimized:** specialized sentence segmentation with context preservation, ideal for BERT/BETO fine-tuning.
- **📦 Zero-Dependency Runtime:** Just `npm install`. No Python required.

## Installation

```bash
npm install bgustdown
```

## Quick Start

### Library Usage (Markdown Conversion)

```javascript
const { Bgustdown } = require('bgustdown');

async function main() {
  const client = new Bgustdown();
  
  // Fast conversion to clean Markdown
  const markdown = await client.convert('./report.pdf');
  console.log(markdown);
}

main();
```

### CLI Usage (Universal Commands)

Once installed, you can use the `bgustdown` command directly:

```bash
# Convert a document to Markdown
npx bgustdown convert ./my-document.pdf

# Prepare a dataset for NLP (Clean segmentation & JSON output)
npx bgustdown prepare ./legal-text.docx
```

## Attribution

This project is conceptually inspired by Microsoft's **MarkItDown**. Re-engineered in Rust for extreme performance and AI specialization.

## License

MIT License.

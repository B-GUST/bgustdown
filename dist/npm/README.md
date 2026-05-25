# bgustdown (Node.js/NPM)

**The definitive high-performance document engine for the AI era.**

`bgustdown` is a native Node.js library written in **Rust**, designed to convert complex documents into clean Markdown and NLP-ready datasets with extreme speed.

## Key Features

- **⚡ Blazing Fast:** Real parallel processing powered by Rust & Tokio.
- **📊 Apache Arrow Integration:** Optimal handling of massive XLSX and CSV files.
- **🧠 AI-Ready Output:** Built-in semantic cleaning and sentence segmentation for BERT/BETO fine-tuning.
- **📦 Zero Python Dependency:** Native binaries pre-compiled via NAPI-RS.

## Installation

```bash
npm install bgustdown
```

## Quick Start

```javascript
const { Bgustdown } = require('bgustdown');
const client = new Bgustdown();

async function main() {
  // Convert any document to Markdown
  const markdown = await client.convert('my-document.pdf');
  console.log(markdown);
}
main();
```

## CLI Usage

```bash
npx bgustdown convert ./file.docx
```

## Version
**Current Stable:** 0.1.2

---
Built with ❤️ by **B-GUST**. Official docs at [bgustdown.lat](https://bgustdown.lat)

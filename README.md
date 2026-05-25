# 🚀 bgustdown

<p align="center">
  <b>The definitive high-performance document engine for the AI era.</b><br>
  <i>Convert PDF, DOCX, and XLSX to clean Markdown & NLP datasets in milliseconds.</i>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/bgustdown"><img src="https://img.shields.io/npm/v/bgustdown.svg?style=flat-square" alt="NPM Version"></a>
  <img src="https://img.shields.io/badge/version-0.1.2-orange.svg?style=flat-square" alt="Stable Version">
  <img src="https://img.shields.io/badge/language-rust-orange.svg?style=flat-square" alt="Language">
  <img src="https://img.shields.io/badge/license-MIT-black.svg?style=flat-square" alt="License">
  <a href="https://doi.org/10.5281/zenodo.20090926"><img src="https://img.shields.io/badge/doi-10.5281/zenodo.20090926-blue.svg?style=flat-square" alt="DOI"></a>
</p>

---

## 💡 The Vision

**bgustdown** is a high-performance data engineering tool built in **Rust**. It eliminates the performance bottlenecks in AI data pipelines by providing ultra-fast document conversion, semantic cleaning, and structural precision.

## 📂 Project Structure

| Directory | Description |
| :--- | :--- |
| `src/` | Core engine source code (Rust). |
| `dist/crates/` | Rust-native distribution files for Crates.io. |
| `dist/npm/` | Node.js native bindings and CLI for NPM. |
| `dist/skill/` | AI Skill installer and instructions for LLM Agents. |
| `scripts/` | Helper scripts for installation and automation. |

## 🛠 Installation & Contribution

### 1. As an AI Skill (Standardized)
```bash
npx skill add https://github.com/B-GUST/bgustdown
```

### 2. For Developers (Git Clone)
If you want to contribute or build the engine from source:
```bash
git clone https://github.com/B-GUST/bgustdown.git
cd bgustdown
npm install && npm run build
```

## 🧠 AI Skill / CLI Manual

| Command | Action |
| :--- | :--- |
| `npx bgustdown convert <path>` | Universal conversion to Markdown. |
| `npx bgustdown prepare <path>` | Semantic NLP Dataset Preparation. |

## 🏗 Technology Stack

- **Engine:** Rust + Tokio.
- **Data Layer:** Apache Arrow.
- **Packaging:** NAPI-RS (Node.js) & Cargo (Rust).
- **Ethics:** Conceptually inspired by Microsoft's MarkItDown.

---
<p align="center">
  <b>Version: 0.1.2 (Stable)</b> | <a href="https://bgustdown.lat">bgustdown.lat</a>
</p>

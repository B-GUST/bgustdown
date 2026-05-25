# 🚀 bgustdown

<p align="center">
  <b>The definitive high-performance document engine for the AI era.</b><br>
  <i>Convert PDF, DOCX, and XLSX to clean Markdown & NLP datasets in milliseconds.</i>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/bgustdown"><img src="https://img.shields.io/npm/v/bgustdown.svg?style=flat-square" alt="NPM Version"></a>
  <a href="https://crates.io/crates/bgustdown"><img src="https://img.shields.io/crates/v/bgustdown.svg?style=flat-square" alt="Crates Version"></a>
  <img src="https://img.shields.io/badge/version-0.1.2-orange.svg?style=flat-square" alt="Stable Version">
  <a href="https://doi.org/10.5281/zenodo.20090926"><img src="https://img.shields.io/badge/doi-10.5281/zenodo.20090926-blue.svg?style=flat-square" alt="DOI"></a>
</p>

---

## 💡 The Vision

**bgustdown** is an industrial-grade data engineering powerhouse built from the ground up in **Rust**. It eliminates the performance bottlenecks in AI data pipelines, providing the speed and semantic precision required for production-grade RAG and NLP fine-tuning.

## 🏛️ Repository Architecture

This repository uses a **multi-branch distribution strategy** to keep the core development separated from production-ready packages.

| Branch | Target Ecosystem | Description |
| :--- | :--- | :--- |
| `main` | **Source / Dev** | The raw Rust source code and development manifests. |
| `npm` | **Node.js / NPM** | Production-ready NAPI bindings and CLI tool. |
| `crates` | **Rust / Cargo** | Pure Rust library distribution for crates.io. |
| `skill` | **AI Agents** | Native AI Skill installer and capabilities manifest. |

## 🛠 Installation & Usage

### For Developers (Git Clone)
If you want to contribute or build the engine from source:
```bash
git clone https://github.com/B-GUST/bgustdown.git
cd bgustdown
npm install && npm run build
```

### As an AI Skill (Agent Native)
```bash
npx skill add https://github.com/B-GUST/bgustdown
```

### Library Installation
- **Node.js:** `npm install bgustdown`
- **Rust:** `cargo add bgustdown`

## 🧠 Command Reference (CLI/Skill)

| Command | Action |
| :--- | :--- |
| `npx bgustdown convert <path>` | Universal conversion to Markdown. |
| `npx bgustdown prepare <path>` | Semantic NLP Dataset Preparation. |

---
<p align="center">
  <b>Version: 0.1.2 (Stable)</b> | Official Docs: <a href="https://bgustdown.lat">bgustdown.lat</a>
</p>

# 🚀 bgustdown

<p align="center">
  <b>The definitive high-performance document engine for the AI era.</b><br>
  <i>Convert PDF, DOCX, and XLSX to clean Markdown & NLP datasets in milliseconds.</i>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/bgustdown"><img src="https://img.shields.io/npm/v/bgustdown.svg?style=flat-square" alt="NPM Version"></a>
  <img src="https://img.shields.io/badge/version-0.1.1-orange.svg?style=flat-square" alt="Stable Version">
  <img src="https://img.shields.io/badge/language-rust-orange.svg?style=flat-square" alt="Language">
  <img src="https://img.shields.io/badge/license-MIT-black.svg?style=flat-square" alt="License">
</p>

---

## 💡 The Vision

**bgustdown** is a high-performance data engineering tool built in **Rust** (v0.1.1).

## 🛠 Instalación y Setup

Ofrecemos tres formas de integrar bgustdown en tu ecosistema:

### 1. Como Librería (NPM)
Ideal para desarrolladores de Node.js/TypeScript.
```bash
npm install bgustdown
```

### 2. Como AI Skill (Estandarizado)
Para agentes IA que soportan la arquitectura de plugins/skills.
```bash
npx skill add https://github.com/B-GUST/bgustdown
```

### 3. Para Contribuyentes (Git Clone)
Si deseas mejorar el core en Rust o ejecutar pruebas locales.
```bash
git clone https://github.com/B-GUST/bgustdown.git
cd bgustdown/bgustdown
npm install && npm run build
```

## 🧠 Manual de la Skill / CLI

Cualquier usuario o agente puede ejecutar los siguientes comandos una vez instalada la herramienta:

| Comando | Acción | Salida |
| :--- | :--- | :--- |
| `npx bgustdown convert <path>` | Conversión universal a Markdown. | String (Markdown) |
| `npx bgustdown prepare <path>` | Segmentación para NLP (BERT/BETO). | JSON (Array) |

## 🏗 Arquitectura y Tecnología

- **Engine:** Rust Nativo + Tokio (Concurrencia real).
- **Tabular Layer:** Apache Arrow para máxima eficiencia de memoria.
- **Bindings:** NAPI-RS para una integración perfecta con Node.js.
- **Inspiración:** Basado conceptualmente en Microsoft MarkItDown.

---
<p align="center">
  <b>Version: 0.1.1 (Stable)</b> | <a href="https://bgustdown.lat">bgustdown.lat</a>
</p>

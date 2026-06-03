---
name: bgustdown
description: Engine for high-performance document ingestion, OCR, and semantic NLP dataset preparation.
tools:
  - name: convert_file
    description: Convert complex local files (PDF, DOCX, XLSX, ODT, CSV) into clean, semantic Markdown.
    arguments:
      path:
        type: string
        description: The absolute or relative file path to the local document.
        required: true
  - name: prepare_nlp_data
    description: Segment and build structured sentences from markdown or plain text, optimized for NLP datasets and RAG.
    arguments:
      text:
        type: string
        description: The raw text or markdown to segment.
        required: true
      source:
        type: string
        description: A label identifying the source repository or file name.
        required: false
      domain:
        type: string
        description: The semantic domain of the text (e.g. legal, financial, medical).
        required: false
---

# Claude Agent Skill: bgustdown

`bgustdown` is an industrial-grade document ingestion and data engineering skill for AI Agents. It provides native, lightning-fast conversion of complex business documents into clean, LLM-ready Markdown and NLP training datasets.

## 🛠️ Tool Integration Guide

### 1. `convert_file`
Use this tool whenever the user provides a local document file (e.g. `.pdf`, `.docx`, `.xlsx`, `.odt`, `.csv`) and wants to extract its contents or convert it to markdown format.

* **Usage Example:**
  ```javascript
  // When requested to convert a local file:
  await tools.convert_file({ path: "./proposals/annual_report.pdf" });
  ```

### 2. `prepare_nlp_data`
Use this tool to clean up raw text or convert a document into structured, segmented sentences for dataset training or embedding generation in vector stores (RAG).

* **Usage Example:**
  ```javascript
  // Segmenting clean markdown text into structured sentences:
  await tools.prepare_nlp_data({
    text: "## Section 1\nThis is a sentence. And here is another one.",
    source: "proposal-doc",
    domain: "corporate"
  });
  ```

## 🧠 Instruction Set for Claude

When executing this skill, adhere to the following rules:

1. **OCR Local fallback:** If a converted document contains images or scan layers, the native OCR engine will run automatically. Do not attempt to run external OCR unless the native convert fails.
2. **Tabular formatting:** When converting Excel or CSV files, rely on the integrated Apache Arrow engine output which represents sheets as beautifully formatted markdown tables. Ensure headers are preserved.
3. **Chunking & RAG pipeline:** Keep document headers (`#`, `##`, `###`) intact during post-processing to allow vector databases to chunk content along natural semantic boundaries.

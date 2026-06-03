# bgustdown 🚀

<p align="center">
  <b>El Motor Definitivo de Ingesta de Documentos y Preparación de Datasets NLP de Alto Rendimiento.</b><br>
  <i>Convierte PDF, DOCX, XLSX, ODT y CSV a Markdown Limpio y Estructuras Arrow en Milisegundos.</i>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/bgustdown"><img src="https://img.shields.io/npm/v/bgustdown.svg?style=flat-square" alt="NPM Version"></a>
  <a href="https://crates.io/crates/bgustdown"><img src="https://img.shields.io/crates/v/bgustdown.svg?style=flat-square" alt="Crates Version"></a>
  <img src="https://img.shields.io/badge/version-0.1.3-orange.svg?style=flat-square" alt="Stable Version">
  <a href="https://doi.org/10.5281/zenodo.20090926"><img src="https://img.shields.io/badge/doi-10.5281/zenodo.20090926-blue.svg?style=flat-square" alt="DOI"></a>
</p>

---

## 💡 La Visión

`bgustdown` es una solución de ingeniería de datos de nivel industrial construida desde cero en **Rust**. Está diseñada para eliminar los cuellos de botella de rendimiento en pipelines de IA, RAG (Retrieval-Augmented Generation) y fine-tuning de LLMs, transformando archivos binarios corporativos complejos en Markdown estructurado y conjuntos de datos semánticos sin latencias innecesarias ni dependencias pesadas de runtime.

---

## 🌟 Características Clave

*   **Motor de Ingesta Multi-Formato Ultra-Rápido (Rust):**
    *   **PDF:** Extracción estructurada y limpia de texto y capas vectoriales usando `lopdf` and `pdf-extract`.
    *   **Office (DOCX / ODT / XLSX):** Conversión directa de esquemas XML (a través de `quick-xml` y `calamine`) a Markdown, preservando tablas, listas y jerarquías de cabeceras.
*   **Formato de Tablas Basado en Apache Arrow:**
    *   Representación interna de datos tabulares (CSV y hojas XLSX) usando **Apache Arrow**, lo que permite operaciones en memoria ultrarrápidas y conversiones de esquemas sin copia de memoria (Zero-Copy).
*   **Procesamiento de Texto y Tokenización Local:**
    *   Integración nativa con `tokenizers` de Hugging Face y modelos locales `candle-core` para segmentación semántica de oraciones orientada a entrenamiento RAG y NLP.
*   **Soporte NAPI-RS Nativo:**
    *   Bindings dinámicos y de alto rendimiento compilados en C/C++ cargados directamente por Node.js sin sobrecoste de llamadas externas.

---

## 🏗️ Arquitectura del Repositorio

El proyecto ha consolidado su arquitectura monorrama en `main`, centralizando el desarrollo del core y organizando las configuraciones y manifiestos de distribución en el directorio `docs/`:

```
                           [ bgustdown Core ]
                                   │
      ┌────────────────────────────┼────────────────────────────┐
      ▼ (Librería Rust)            ▼ (Bindings Node/NAPI)       ▼ (Skill para Agentes)
 [ Canal Crates.io (v0.1.3) ]   [ Canal NPM / CLI (v0.1.2) ] [ Canal Skill de IA ]
  - docs/crates/                 - docs/npm/                  - docs/skill/
  - Importable como rlib         - Binario cdylib (.node)     - install_skill.sh
```

---

## 📦 Canales de Distribución

### 1. Canal Rust (Crates.io) 🦀
*   **Versión Activa:** `v0.1.3`
*   **Tipo de Biblioteca:** Estática (`rlib`).
*   **Uso:** Añade `bgustdown` a tu proyecto Rust:
    ```toml
    [dependencies]
    bgustdown = "0.1.3"
    ```
*   Ver detalles de publicación en: [docs/crates/README.md](./docs/crates/README.md).

### 2. Canal Node.js & CLI (NPM) 🟢
*   **Versión Activa:** `v0.1.2`
*   **Tipo de Biblioteca:** Dinámica (`cdylib` mediante NAPI-RS).
*   **Instalación:**
    ```bash
    npm install -g bgustdown
    ```
*   Ver detalles de publicación en: [docs/npm/README.md](./docs/npm/README.md).

### 3. Canal Skill para Agentes de IA 🧠
*   Integra las herramientas `convert_file` y `prepare_nlp_data` directamente en agentes autónomos (como Claude).
*   Ver detalles y manifiesto en: [docs/skill/SKILL.md](./docs/skill/SKILL.md).

---

## 🛠️ Instalación y Compilación de Desarrollo

Si deseas compilar la librería dinámica y probar el CLI en tu entorno de desarrollo local:

1.  **Clonar el repositorio:**
    ```bash
    git clone https://github.com/B-GUST/bgustdown.git
    cd bgustdown
    ```

2.  **Instalar dependencias de desarrollo e iniciar compilación:**
    ```bash
    npm install
    npm run build
    ```

3.  **Ejecutar el CLI:**
    ```bash
    node cli.js convert ruta/a/tu/archivo.pdf
    ```

---

## 🤖 Guía de Uso del CLI Global

Una vez enlazado o instalado globalmente:

| Comando | Acción |
| :--- | :--- |
| `bgustdown convert <file_path>` | Conversión universal a Markdown. |
| `bgustdown prepare <file_path>` | Segmentación y estructuración en JSON de datos NLP. |

---

## 📜 Licencia y Créditos

Este proyecto se distribuye bajo la licencia MIT. Consulta el archivo `CREDITS.md` para atribuciones a las librerías de terceros (Apache Arrow, Calamine, NAPI-RS, Candle).

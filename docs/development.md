# Guía de Desarrollo Local y Pruebas

Esta guía está destinada a desarrolladores que deseen compilar, depurar, ejecutar pruebas unitarias y medir el rendimiento del motor `bgustdown`.

---

## 🛠️ 1. Entorno de Desarrollo Local

### Requisitos Técnicos
Asegúrate de contar con las siguientes herramientas en tu sistema operativo:
* **Compilador C / GCC:** Necesario para compilar dependencias nativas de bajo nivel.
* **Rust & Cargo (v1.75+):** Cadena de herramientas nativa para el motor en Rust.
* **Node.js (v16+) y npm:** Para la construcción de bindings.

---

## 🧪 2. Ejecución de la Suite de Pruebas

El proyecto cuenta con una suite completa de pruebas para garantizar que los parsers y algoritmos de segmentación semántica funcionen de forma estable.

### Pruebas en Rust (Crate Principal)
Para ejecutar todas las pruebas unitarias y de integración del motor en Rust:
```bash
# Ejecutar todas las pruebas en modo depuración
cargo test

# Ejecutar una prueba específica por su nombre
cargo test test_excel_conversion
```

### Pruebas de Integración en Node.js
Si has realizado cambios en los bindings de JS, puedes ejecutar los scripts de prueba nativos:
```bash
npm run test
```

---

## 🏎️ 3. Lints y Estilo de Código

Es obligatorio validar el estilo de código antes de proponer cualquier cambio al repositorio principal para garantizar consistencia estructural:

```bash
# 1. Comprobar formateo de código
cargo fmt --all -- --check

# 2. Corregir automáticamente formateo inadecuado
cargo fmt --all

# 3. Validar con el linter de Rust
cargo clippy --all-targets --all-features -- -D warnings
```

---

## 📊 4. Benchmarking y Rendimiento

El motor incluye configuraciones para medir los tiempos de procesamiento en milisegundos para grandes volúmenes de documentos:

```bash
# Ejecutar benchmarks en modo release
cargo bench
```

Esto compilará el código con optimizaciones máximas y ejecutará análisis estadísticos de rendimiento en la conversión de estructuras tabulares, PDFs y DOCX.

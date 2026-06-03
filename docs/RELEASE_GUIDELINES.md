# Guía de Versionado y Protocolo de Lanzamiento (SemVer & Multi-Distribución) 🚀

Este documento detalla el estándar y protocolo de publicación para **`bgustdown`** con el fin de asegurar la estabilidad, sincronización y compatibilidad del core del ecosistema tanto en **crates.io** como en **NPM**.

---

## 📐 1. Estrategia de Versionado (SemVer)

`bgustdown` sigue estrictamente el estándar de **Semantic Versioning (SemVer)**: `MAJOR.MINOR.PATCH`.

*   **PATCH (x.y.PATCH):** Corrección de errores que no cambian el comportamiento del API ni rompen compatibilidad (ej: corrección en conversor de PDFs).
*   **MINOR (x.MINOR.z):** Nuevas funcionalidades compatibles hacia atrás (ej: añadir soporte para nuevos tipos de documentos corporativos) o cambios estructurales como la unificación de dependencias.
*   **MAJOR (MAJOR.y.z):** Cambios incompatibles con versiones anteriores en la API pública de Rust o Node.js.

---

## 🏛️ 2. Estructura Híbrida de Distribución

El proyecto está diseñado bajo una arquitectura de **compilación dual** que se divide en dos canales oficiales:

### A. Canal Rust (Crates.io) 🦀
*   **Destino:** Consumidores de Rust que importan `bgustdown` como biblioteca nativa.
*   **Tipo de compilación:** Estática (`rlib`).
*   **Manifiesto de Publicación:** [docs/crates/Cargo.toml](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/docs/crates/Cargo.toml).
    > [!NOTE]
    > Este manifiesto excluye de forma estricta las dependencias de bindings dynamic (`cdylib` nativas de NAPI) y los bloques de workspaces para garantizar que se compile de forma aislada y ligera en crates.io.

### B. Canal Node.js (NPM) 🟢
*   **Destino:** Desarrolladores de Node.js y ejecución en CLI.
*   **Tipo de compilación:** Dinámica (`cdylib` + NAPI-RS bindings).
*   **Manifiesto de Publicación:** [package.json](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/package.json) (o [docs/npm/package.json](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/docs/npm/package.json)).
*   **Dependencias de plataforma:** Las librerías de plataforma (`optionalDependencies`) deben coincidir exactamente en versión con el paquete core para evitar incompatibilidades binarias y fallos del ABI al cargar la extensión nativa `.node`.

---

## 🛠️ 3. Protocolo de Lanzamiento Paso a Paso (Release Pipeline)

Sigue esta lista de verificación rigurosa para realizar un nuevo despliegue de versión (ejemplo para versión `0.1.4`):

### Paso 1: Sincronización de Versión en Manifiestos
Asegúrate de que la versión esté configurada uniformemente en todos los archivos de configuración:
1.  **Cargo.toml principal:** [Cargo.toml](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/Cargo.toml) -> `version = "0.1.4"`
2.  **Cargo.toml de Crates:** [docs/crates/Cargo.toml](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/docs/crates/Cargo.toml) -> `version = "0.1.4"`
3.  **package.json principal:** [package.json](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/package.json) -> `"version": "0.1.4"`
4.  **package.json de NPM:** [docs/npm/package.json](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/docs/npm/package.json) -> `"version": "0.1.4"`
5.  **Optional Dependencies:** En [package.json](file:///home/august/code/bgust-nlp-ecosystem/bgustdown/package.json), actualiza los paquetes binarios específicos de plataforma a `"0.1.4"`.

### Paso 2: Validación Local
Antes de publicar, ejecuta y valida las compilaciones en tu máquina de desarrollo:
```bash
# 1. Validar compilación de Rust
cargo check

# 2. Compilar extensión nativa NAPI-RS de desarrollo
npm run build

# 3. Ejecutar pruebas unitarias e integración de Node
npm test
```

### Paso 3: Publicación en Crates.io
Una vez autenticado mediante `cargo login`:
```bash
# Se recomienda publicar desde el manifiesto limpio de crates
cargo publish --manifest-path docs/crates/Cargo.toml
```

### Paso 4: Publicación en NPM
Una vez autenticado mediante `npm login`:
1.  Genera los artefactos de compilación nativos para cada plataforma soportada.
2.  Publica los paquetes de arquitectura y finalmente el paquete core:
```bash
# Generar los artefactos finales
napi build --platform --release

# Publicar el paquete core (esto también disparará la verificación de napi-rs)
npm publish
```

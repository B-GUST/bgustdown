# Guía de Resolución de Problemas (Troubleshooting)

Este documento recopila las soluciones a los errores comunes, advertencias y fallos conocidos durante el desarrollo, compilación o ejecución del motor `bgustdown`.

---

## 🛠️ 1. Errores de Compilación en Cargo (Rust)

### A. Error: "current package believes it's in a workspace when it's not"
Este problema ocurre cuando intentas compilar un subproyecto individual de `bgustdown` dentro de un monorepo que define un `Cargo.toml` de workspace en el directorio padre, pero el subproyecto no está listado explícitamente en el arreglo de `workspace.members`.

* **Solución:**
  Asegúrate de incluir la directiva `[workspace]` vacía en el archivo `Cargo.toml` del subproyecto (ej: en `bgustdown-crates/Cargo.toml`). Esto le indica a Cargo que trate la carpeta como un workspace independiente, aislando su contexto de compilación del padre:
  ```toml
  [workspace]
  ```

### B. Error en dependencias de compilación al enlazar bindings NAPI
Si obtienes errores relacionados con `napi-build` o en la ejecución del script `build.rs`:

* **Solución:**
  * Asegúrate de que las dependencias necesarias de C estén instaladas en tu sistema operativo (`build-essential` en Ubuntu/Debian).
  * Si solo deseas compilar `bgustdown` como una librería de Rust estándar sin dependencias de Node.js, verifica que has configurado el crate-type correcto en el `Cargo.toml`:
    ```toml
    [lib]
    crate-type = ["rlib"]
    ```

---

## 🟢 2. Problemas con los Bindings de Node.js (NPM)

### A. Error: "Cannot find module './bgustdown.<platform>.node'"
Este error ocurre cuando Node.js no puede encontrar el binario nativo compilado para tu plataforma y arquitectura de CPU específica.

* **Solución:**
  * Re-compila el binario localmente corriendo `npm run build` en el directorio del proyecto para regenerar el archivo `.node` correspondiente a tu arquitectura actual.
  * Si estás distribuyendo la aplicación, asegúrate de que el paquete NPM incluya los módulos precompilados opcionales para la plataforma de destino o compila en el entorno de despliegue final.

---

## 🤖 3. Problemas de Inferencia OCR y Carga de Modelos

### A. Fallo de inicialización o falta de archivos del modelo
Al instanciar `Bgustdown::new()`, el sistema busca localmente los archivos del modelo para la tokenización y la inferencia OCR offline. Si no se encuentran o están corruptos:

* **Solución:**
  * Asegúrate de que los archivos de pesos del modelo OCR y del tokenizador estén ubicados en la ruta esperada de tu sistema de archivos (por defecto, `models/nlp-core/`).
  * Si estás operando en un entorno completamente desconectado de red, descarga previamente los modelos correspondientes (ej: `SmolLM2-135M` o similares) desde Hugging Face y colócalos en la carpeta de caché local del proyecto.

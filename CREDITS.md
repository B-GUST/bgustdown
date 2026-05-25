# Créditos y Atribuciones

Este proyecto, **bgustdown**, ha sido desarrollado con inspiración y basándose en el diseño conceptual del proyecto **MarkItDown** de Microsoft Corporation.

## Atribución Original
- **Proyecto:** [MarkItDown](https://github.com/microsoft/markitdown)
- **Autor Original:** Microsoft Corporation
- **Licencia:** MIT

## Cambios y Mejoras en bgustdown
- **Re-implementación en Rust:** Todo el núcleo de conversión ha sido portado a Rust para maximizar el rendimiento y la seguridad de memoria.
- **Bindings Nativa para Node.js:** Uso de `napi-rs` para permitir un uso transparente en el ecosistema NPM sin dependencias de Python.
- **Optimización de Parsers:** Uso de librerías de alto rendimiento como `calamine` (Excel/ODS), `rdocx` (Word/ODT) y `pdfsink-rs` (PDF).
- **OCR Local Integrado:** Inclusión de modelos Tiny Vision (vía `candle-core`) para procesamiento OCR autónomo.

Agradecemos a Microsoft y a la comunidad de MarkItDown por sentar las bases de esta herramienta.

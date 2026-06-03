# Referencia de la API Pública

Este documento detalla la interfaz de programación pública expuesta por el motor `bgustdown` tanto en su especificación nativa de **Rust** como en su mapeo de bindings para **Node.js / TypeScript**.

---

## 🦀 1. API Nativa de Rust

La API de Rust devuelve tipos nativos estándar y aprovecha el sistema de tipos asíncronos (`async/await`) de Tokio.

### `Bgustdown` (Estructura Principal)

La estructura central encapsula el motor de inferencia OCR y los optimizadores.

#### `Bgustdown::new() -> Result<Self, Error>`
Instancia un nuevo cliente de conversión. Carga en memoria las configuraciones del extractor y valida el estado de inicialización local del motor de inferencia/OCR.

* **Retorno:**
  * `Ok(Bgustdown)`: Instancia inicializada.
  * `Err(Error)`: Error de configuración o falta de recursos locales.

```rust
use bgustdown::Bgustdown;

let engine = Bgustdown::new()?;
```

#### `Bgustdown::convert(&self, path: String) -> Result<String, Error>`
Método asíncrono para convertir cualquier documento estructurado a Markdown limpio.
* **Argumentos:**
  * `path`: Ruta absoluta o relativa al documento (soporta `.pdf`, `.docx`, `.odt`, `.xlsx`, `.xls`, `.csv`).
* **Retorno:**
  * `Ok(String)`: Documento convertido en Markdown semántico optimizado.

```rust
let markdown = engine.convert("mi_reporte.pdf".to_string()).await?;
```

#### `Bgustdown::prepare_training_data(&self, text: String, source: String, domain: String) -> Result<Vec<String>, Error>`
Segmenta texto limpio en oraciones optimizadas para alimentar pipelines de entrenamiento NLP o pares de Next Sentence Prediction (NSP).
* **Argumentos:**
  * `text`: Texto fuente limpio en Markdown o texto plano.
  * `source`: Identificador de origen de datos (con fines de etiquetado).
  * `domain`: Dominio semántico (ej: `"legal"`, `"financiero"`).
* **Retorno:**
  * `Ok(Vec<String>)`: Vector de oraciones curadas y segmentadas.

---

## 🟢 2. API de Node.js / TypeScript

Los bindings de JavaScript se exponen a través de clases nativas con rendimiento asíncrono no bloqueante en el Thread Pool de libuv.

### Definición de Tipos (`index.d.ts`)

```typescript
export class Bgustdown {
  constructor();
  convert(path: string): Promise<string>;
  prepareTrainingData(text: string, source: string, domain: string): string[];
}

export function convertFile(path: string): Promise<string>;
export function convertText(input: string): string;
```

### Ejemplos de Integración en JavaScript/TypeScript

#### Conversión Asíncrona Directa:
```javascript
const { convertFile } = require('bgustdown');

async function run() {
  try {
    const md = await convertFile('./datos.xlsx');
    console.log(md);
  } catch (err) {
    console.error("Fallo al convertir archivo:", err);
  }
}
run();
```

#### Uso de la Clase Orientada a Objetos:
```typescript
import { Bgustdown } from 'bgustdown';

const client = new Bgustdown();

async function main() {
  // Convertir documento con limpieza semántica activada por defecto
  const markdown = await client.convert('factura.pdf');
  console.log("Markdown obtenido:", markdown);

  // Preparar dataset para procesamiento NLP posterior
  const sentences = client.prepareTrainingData(markdown, 'local-pdf', 'billing');
  console.log(`Se extrajeron ${sentences.length} oraciones para entrenamiento.`);
}
main();
```

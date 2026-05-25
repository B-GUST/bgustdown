# 🧠 AI Skill: bgustdown

Esta skill proporciona capacidades de ingeniería de datos de alto rendimiento para procesar documentos complejos y convertirlos en formatos aptos para IA.

## Capacidades

- **Conversión Universal:** Transforma PDF, DOCX, XLSX, ODT, ODS y CSV a Markdown estructurado en milisegundos.
- **Preparación de Datasets:** Limpia semánticamente el ruido y segmenta oraciones para entrenamiento de modelos (BERT/BETO).
- **Extracción de Tablas:** Reconstruye tablas complejas de Excel y Word directamente en Markdown.

## Instrucciones para el Agente

Cuando un usuario necesite leer el contenido de un archivo local que no sea texto plano (como un PDF o Excel) o preparar datos para entrenamiento:

1. Utiliza el comando `npx bgustdown convert <path>` para extraer el contenido en Markdown.
2. Utiliza el comando `npx bgustdown prepare <path>` para obtener un dataset segmentado en oraciones.

## Herramientas (Comandos)

| Comando | Descripción | Ejemplo |
| :--- | :--- | :--- |
| `npx bgustdown convert <file>` | Convierte documento a Markdown limpio. | `npx bgustdown convert reporte.pdf` |
| `npx bgustdown prepare <file>` | Genera lista de oraciones limpias para NLP. | `npx bgustdown prepare ley.docx` |

---
*Skill desarrollada por B-GUST — High Performance Data Engineering.*

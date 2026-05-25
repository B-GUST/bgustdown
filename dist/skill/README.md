# 🧠 AI Skill: bgustdown

Esta skill proporciona capacidades de ingeniería de datos de alto rendimiento para que agentes de IA procesen documentos locales complejos (PDF, DOCX, XLSX).

## Instalación de la Skill

Para instalar esta herramienta en tu agente:

```bash
npx skill add https://github.com/B-GUST/bgustdown
```

## Capacidades para la IA

- **Extracción Estructural:** Recupera texto y tablas preservando la jerarquía.
- **Preparación de Datasets:** Genera arrays de oraciones segmentadas en JSON para tareas de NLP.
- **Limpieza de Ruido:** Elimina artefactos como números de página y cabeceras antes del procesamiento.

## Comandos Disponibles

| Comando | Acción |
| :--- | :--- |
| `npx bgustdown convert <file>` | Markdown estructurado. |
| `npx bgustdown prepare <file>` | Dataset JSON segmentado. |

## Versión
**Estable:** 0.1.2

---
Desarrollado por **B-GUST** para la comunidad de IA Open Source.

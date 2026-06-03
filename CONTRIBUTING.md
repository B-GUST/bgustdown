# Guía de Contribución (`CONTRIBUTING.md`)

¡Gracias por tu interés en contribuir a **bgustdown**! Este es un proyecto de código abierto industrial y damos la bienvenida a desarrolladores, investigadores de NLP e ingenieros de datos para mejorar el motor.

Para asegurar un desarrollo coordinado, estable y limpio, solicitamos a todos los colaboradores seguir las siguientes pautas al proponer cambios a través de **Pull Requests (PR)** en GitHub.

---

## 🗺️ 1. Flujo de Trabajo en Git (Git Flow)

El proyecto utiliza un sistema estructurado de ramas basado en ramas de características (`feature branches`).

### Paso 1: Crear una rama de desarrollo
Crea una rama descriptiva desde `main` (o `master` según corresponda) utilizando nombres en inglés que expliquen el cambio:
```bash
git checkout -b feature/your-awesome-feature
# o si es un error:
git checkout -b fix/resolve-some-bug
```

### Paso 2: Desarrollar e integrar pruebas locales
Asegúrate de que tus modificaciones compilen y pasen la suite de validación integrada antes de hacer cualquier commit.
```bash
cargo fmt --all      # Formatear el código bajo el estándar Rustfmt
cargo clippy         # Ejecutar el linter para detectar antipatrones
cargo test           # Ejecutar todas las pruebas unitarias y de integración
```

### Paso 3: Mantener la rama sincronizada
Antes de enviar tu propuesta, haz un rebase o merge de los últimos cambios de `main` para evitar conflictos:
```bash
git fetch origin
git merge origin/main
```

---

## 📝 2. Convención de Commits (Conventional Commits)

Seguimos el estándar de **Conventional Commits** para mantener un historial limpio y estructurado que facilite la generación automática de Changelogs y versionados semánticos.

Cada mensaje de commit debe tener el siguiente formato:
```
<tipo>(<ámbito opcional>): <descripción breve en imperativo y minúsculas>

[cuerpo opcional detallando el motivo y contexto]

[pie opcional con referencias a tickets/issues, ej: Closes #123]
```

### Tipos Permitidos:
* **`feat`**: Nueva funcionalidad (ej: `feat(ocr): add support for handwritten text extraction`).
* **`fix`**: Corrección de un error (ej: `fix(excel): resolve out-of-bounds in cell parsing`).
* **`docs`**: Cambios en la documentación (ej: `docs: update install guide for macOS ARM`).
* **`style`**: Cambios cosméticos que no afectan la lógica (espacios, formateo, punto y coma).
* **`refactor`**: Reestructuración de código que no corrige un bug ni añade una función (ej: `refactor(arrow): reuse arrow buffers in csv parser`).
* **`test`**: Añadir o corregir pruebas existentes (ej: `test(docx): add integration tests for tables`).
* **`chore`**: Tareas de mantenimiento, actualización de dependencias o configuraciones del build system.

---

## 🚀 3. Guía para Crear Pull Requests (PR)

Cuando tu código esté listo y subido a tu fork o rama remota en GitHub:

1. **Abre un Pull Request** apuntando a la rama `main` del repositorio oficial.
2. **Usa la Plantilla de PR** (o describe detalladamente el cambio con la siguiente estructura):
   * **Descripción:** ¿Qué problema resuelve este cambio y cómo lo hace?
   * **Tipo de Cambio:** Marca si es `feat`, `fix`, `docs`, `refactor`, etc.
   * **Validación Realizada:** Especifica qué pruebas ejecutaste localmente para demostrar la estabilidad (ej: `cargo test`, pruebas manuales con archivos PDF, benchmarks de rendimiento).
   * **Issues Relacionados:** Enlaza a los tickets resueltos escribiendo `Closes #numero`.

### Requisitos Técnicos Obligatorios para la Aprobación:
* **Compilación Perfecta:** El código debe compilar en todos los perfiles sin advertencias de Clippy.
* **Sin Regresiones:** El 100% de las pruebas preexistentes deben seguir pasando exitosamente.
* **Nuevas Pruebas:** Si añades una nueva característica o corriges un bug complejo, debes incluir su respectiva prueba unitaria o de integración.
* **Documentación Actualizada:** Si modificas el comportamiento de la API pública o agregas una característica, debes actualizar la documentación en el directorio `docs/` y el archivo `README.md`.

---

## 🤝 4. Código de Conducta e Interacciones

* Mantén una comunicación profesional, respetuosa y constructiva durante la revisión de código.
* La revisión por pares es un proceso de aprendizaje mutuo; agradecemos el feedback constructivo enfocado en la calidad, mantenibilidad y rendimiento del software.

¡Gracias de nuevo por ayudarnos a construir el mejor motor de datos NLP en Rust!

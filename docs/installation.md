# Guía de Instalación y Configuración

`bgustdown` está diseñado como un motor multipropósito y cuenta con métodos específicos de instalación para distintos ecosistemas.

---

## 🦀 1. Instalación en Proyectos Rust (Cargo)

Para usar `bgustdown` de forma nativa en una aplicación Rust como una biblioteca estática (`rlib`):

### Requisitos Previos
* Rust 1.75 o superior instalado (se recomienda la versión estable más reciente).

### Configuración de Dependencias
Añade `bgustdown` directamente a tu archivo `Cargo.toml`:
```toml
[dependencies]
bgustdown = "0.1.3"
```

Alternativamente, ejecuta en tu terminal:
```bash
cargo add bgustdown
```

---

## 🟢 2. Instalación en Proyectos Node.js (NPM)

Si estás desarrollando en JavaScript o TypeScript y deseas aprovechar el rendimiento nativo del motor sin dependencias complejas:

### Requisitos Previos
* Node.js v16 o superior.

### Instalación
Ejecuta el siguiente comando en el directorio de tu proyecto:
```bash
npm install bgustdown
```

Los bindings nativos se compilarán/descargarán de forma automática según el sistema operativo y arquitectura de tu CPU gracias a la infraestructura de `napi-rs`.

---

## 🤖 3. Instalación como Habilidad de IA (Agent-Native Skill)

Si estás integrando `bgustdown` en un framework de agentes autónomos compatibles con especificaciones OpenAPI/Skills:

```bash
npx skill add https://github.com/B-GUST/bgustdown
```

---

## 🛠️ 4. Compilación e Instalación desde el Código Fuente

Si deseas contribuir, realizar pruebas de performance o compilar manualmente el motor optimizado para la arquitectura específica de tu procesador:

### Paso 1: Clonar el Repositorio
```bash
git clone https://github.com/B-GUST/bgustdown.git
cd bgustdown
```

### Paso 2: Compilación de Rust
Compila el crate principal utilizando el perfil de optimización para producción:
```bash
cargo build --release --lib
```
Los archivos compilados resultantes (como librerías estáticas `.rlib`) se ubicarán en `target/release/`.

### Paso 3: Compilación de Bindings de Node.js (Opcional)
Si necesitas compilar localmente los módulos nativos de Node.js:
```bash
npm install
npm run build
```
Esto utilizará `napi-cli` para generar el archivo binario nativo `.node` compatible con tu plataforma actual y los tipos TypeScript asociados (`index.d.ts`).

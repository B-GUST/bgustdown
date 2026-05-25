#!/bin/bash
# bgustdown Skill Installer
# Compatible con npx skill add

SKILL_NAME="bgustdown"
REPO_URL="https://github.com/B-GUST/bgustdown.git"

echo "🚀 Instalando AI Skill: $SKILL_NAME..."

# 1. Clonar o descargar la lógica de la skill
# En un entorno real, npx skill add maneja la descarga, 
# pero aquí dejamos la lógica lista para ser invocada.

npm install -g bgustdown

echo "✅ Skill $SKILL_NAME instalada correctamente v0.1.1"
echo "💡 Puedes usarla con: npx bgustdown convert <file>"

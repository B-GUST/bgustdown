#!/usr/bin/env bash
# ==============================================================================
# bgustdown Skill Installer
# Automates compilation of native Rust code & global Node CLI registration.
# ==============================================================================

set -e

# Curated, harmonious terminal output coloring
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== bgustdown AI Skill Installer ===${NC}"

# Step 1: Detect Build Dependencies
echo -e "\n${BLUE}[1/4] Checking build dependencies...${NC}"

if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed. Please install Node.js (v18+).${NC}"
    exit 1
fi
NODE_VERSION=$(node -v)
echo -e "  - Node.js: ${GREEN}${NODE_VERSION}${NC} (Found)"

if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm is not installed.${NC}"
    exit 1
fi
NPM_VERSION=$(npm -v)
echo -e "  - npm: ${GREEN}${NPM_VERSION}${NC} (Found)"

if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}Warning: Rust compiler (rustc) not found. Attempting to locate cargo...${NC}"
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed. Rust is required to build NAPI bindings.${NC}"
    echo -e "Please install Rust by running: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
CARGO_VERSION=$(cargo --version)
echo -e "  - Cargo/Rust: ${GREEN}${CARGO_VERSION}${NC} (Found)"

# Step 2: Navigate to parent directory (root package) and install development dependencies
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." &> /dev/null && pwd)"

echo -e "\n${BLUE}[2/4] Installing development packages in ${ROOT_DIR}...${NC}"
cd "$ROOT_DIR"
npm install

# Step 3: Compile NAPI Native Rust Bindings
echo -e "\n${BLUE}[3/4] Compiling native Rust library (cdylib)...${NC}"
npm run build

# Step 4: Register Command Globally
echo -e "\n${BLUE}[4/4] Linking CLI tool globally...${NC}"
# Use npm link to register the "bgustdown" command globally using the local bin path
if [ "$EUID" -ne 0 ]; then
    echo -e "${YELLOW}Notice: Attempting to link globally. If this fails due to permissions, re-run with sudo or configure a custom npm directory prefix.${NC}"
fi
npm link

echo -e "\n${GREEN}=== Installation Complete ===${NC}"
echo -e "You can now execute: ${BLUE}bgustdown --help${NC}"

# Verification
if command -v bgustdown &> /dev/null; then
    echo -e "\n${GREEN}Verification Succeeded:${NC}"
    bgustdown --help | head -n 4
else
    echo -e "\n${YELLOW}Verification warning: 'bgustdown' command was linked but is not yet in your current PATH environment variable.${NC}"
    echo -e "Please make sure your npm bin path (e.g. $(npm prefix -g)/bin) is in your PATH."
fi

# File: ~/nairobi-connector-open-source/build_wheel.sh
# Author: Kevin Chege
# Date: 2026-05-06

#!/bin/bash
# nairobi-connector-open-source/build_wheel.sh
set -e

PROJECT_ROOT=$(pwd)
PYTHON_CRATE_DIR="$PROJECT_ROOT/crates/nairobi-python"
REFINERY_CRATE_DIR="$PROJECT_ROOT/crates/nairobi-axum-refinery"
BIN_DEST_DIR="$PYTHON_CRATE_DIR/nairobi_os/bin"

echo "=========================================="
echo "Nairobi OS: Heavy Iron Build Orchestrator"
echo "=========================================="

# 1. Build the Refinery Daemon
echo "Step 1: Compiling Axum Refinery..."
cargo build --release -p nairobi-axum-refinery

# 2. Locate and Prepare Binary
REFINERY_BIN="$PROJECT_ROOT/target/release/nairobi-axum-refinery"

if [ ! -f "$REFINERY_BIN" ]; then
    echo "ERROR: Binary not found at $REFINERY_BIN"
    exit 1
fi

echo "Step 2: Preparing binary for distribution..."
mkdir -p "$BIN_DEST_DIR"
cp "$REFINERY_BIN" "$BIN_DEST_DIR/"
strip "$BIN_DEST_DIR/nairobi-axum-refinery" # Remove debug symbols to save space
chmod +x "$BIN_DEST_DIR/nairobi-axum-refinery"

# 3. Build the Python Wheel
echo "Step 3: Forging the Python Wheel..."
cd "$PYTHON_CRATE_DIR"

# Use maturin from virtual environment if it exists
if [ -f "$PROJECT_ROOT/.venv/bin/maturin" ]; then
    "$PROJECT_ROOT/.venv/bin/maturin" build --release
elif command -v maturin &> /dev/null; then
    maturin build --release
else
    echo "ERROR: maturin not found. Please install it in the virtual environment."
    exit 1
fi

echo "=========================================="
echo "SUCCESS: Wheel forged at $PROJECT_ROOT/target/wheels/"
echo "=========================================="
#!/bin/bash
# nairobi-connector-open-source/build_wheel.sh
set -e

PROJECT_ROOT=$(pwd)
PYTHON_CRATE_DIR="$PROJECT_ROOT/crates/nairobi-python"
BIN_DEST_DIR="$PYTHON_CRATE_DIR/nairobi_os/bin"

echo "=========================================="
echo "Nairobi OS v0.3.0: Heavy Iron Build Orchestrator"
echo "=========================================="

# 1. Build the Microservice Binaries
echo "Step 1: Compiling Axum Refinery..."
cargo build --release -p nairobi-axum-refinery

echo "Step 1b: Compiling Lagos Vision Daemon..."
cargo build --release -p lagos-lite --bin lagos-vision-daemon

# 2. Locate and Prepare Binaries
REFINERY_BIN="$PROJECT_ROOT/target/release/nairobi-axum-refinery"
LAGOS_BIN="$PROJECT_ROOT/target/release/lagos-vision-daemon"

if [ ! -f "$REFINERY_BIN" ]; then
    echo "ERROR: Refinery binary not found at $REFINERY_BIN"
    exit 1
fi

if [ ! -f "$LAGOS_BIN" ]; then
    echo "ERROR: Lagos binary not found at $LAGOS_BIN"
    exit 1
fi

echo "Step 2: Preparing binaries for distribution..."
mkdir -p "$BIN_DEST_DIR"

cp "$REFINERY_BIN" "$BIN_DEST_DIR/"
cp "$LAGOS_BIN" "$BIN_DEST_DIR/"

# Remove debug symbols to save space
strip "$BIN_DEST_DIR/nairobi-axum-refinery"
strip "$BIN_DEST_DIR/lagos-vision-daemon"

chmod +x "$BIN_DEST_DIR/nairobi-axum-refinery"
chmod +x "$BIN_DEST_DIR/lagos-vision-daemon"

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
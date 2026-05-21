#!/bin/bash
# Copyright 2026 Kevin Chege
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# File: ~/nairobi-connector-open-source/build_wheel.sh
# nairobi-connector-open-source/build_wheel.sh
set -e

PROJECT_ROOT=$(pwd)
PYTHON_CRATE_DIR="$PROJECT_ROOT/crates/nairobi-python"
BIN_DEST_DIR="$PYTHON_CRATE_DIR/nairobi_os/bin"

echo "=========================================="
echo "Nairobi OS v0.4.0: Heavy Iron Build Orchestrator"
echo "=========================================="

# 1. Build the Microservice Binaries in parallel
echo "Step 1: Compiling Microservices (Refinery, Lagos & Connector)..."
cargo build --release -p nairobi-axum-refinery -p lagos-lite -p nairobi-connector

# 2. Locate and Prepare Binaries
REFINERY_BIN="$PROJECT_ROOT/target/release/nairobi-axum-refinery"
LAGOS_BIN="$PROJECT_ROOT/target/release/lagos-vision-daemon"
CONNECTOR_BIN="$PROJECT_ROOT/target/release/nairobi-connector"

if [ ! -f "$REFINERY_BIN" ]; then
    echo "ERROR: Refinery binary not found at $REFINERY_BIN"
    exit 1
fi

if [ ! -f "$LAGOS_BIN" ]; then
    echo "ERROR: Lagos binary not found at $LAGOS_BIN"
    exit 1
fi

if [ ! -f "$CONNECTOR_BIN" ]; then
    echo "ERROR: Connector binary not found at $CONNECTOR_BIN"
    exit 1
fi

echo "Step 2: Preparing binaries for distribution..."
mkdir -p "$BIN_DEST_DIR"

cp "$REFINERY_BIN" "$BIN_DEST_DIR/"
cp "$LAGOS_BIN" "$BIN_DEST_DIR/"
cp "$CONNECTOR_BIN" "$BIN_DEST_DIR/"

# Remove debug symbols to save space
strip "$BIN_DEST_DIR/nairobi-axum-refinery"
strip "$BIN_DEST_DIR/lagos-vision-daemon"
strip "$BIN_DEST_DIR/nairobi-connector"

chmod +x "$BIN_DEST_DIR/nairobi-axum-refinery"
chmod +x "$BIN_DEST_DIR/lagos-vision-daemon"
chmod +x "$BIN_DEST_DIR/nairobi-connector"

# 3. Build the Python Wheel
echo "Step 3: Forging the Python Wheel..."
cd "$PYTHON_CRATE_DIR"

# Use maturin from virtual environment if it exists
export PATH="$PROJECT_ROOT/zig_dist:$PATH"
if [ -f "$PROJECT_ROOT/.venv/bin/maturin" ]; then
    "$PROJECT_ROOT/.venv/bin/maturin" build --release --compatibility manylinux2014 --zig
elif command -v maturin &> /dev/null; then
    maturin build --release --compatibility manylinux2014 --zig
else
    echo "ERROR: maturin not found. Please install it in the virtual environment."
    exit 1
fi

echo "=========================================="
echo "SUCCESS: Wheel forged at $PROJECT_ROOT/target/wheels/"
echo "=========================================="

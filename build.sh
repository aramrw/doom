#!/bin/bash

# Configuration
PROJECT_NAME="rsdoom"
BINARY_NAME="librsdoom.dylib"
RUST_DIR="rust"
BIN_DIR="bin"

# Determine build mode
MODE="debug"
CARGO_FLAGS=""

while [[ "$#" -gt 0 ]]; do
    case $1 in
        -r|--release) MODE="release"; CARGO_FLAGS="--release"; shift ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
done

echo "Building Rust extension in $MODE mode..."

# Build the Rust project
cd "$RUST_DIR" || exit 1
if cargo build $CARGO_FLAGS; then
    echo "Build successful."
else
    echo "Build failed."
    exit 1
fi

# Ensure destination directory exists
mkdir -p "../$BIN_DIR"

# Copy the binary to the Godot bin folder
cp "target/$MODE/$BINARY_NAME" "../$BIN_DIR/"
echo "Copied target/$MODE/$BINARY_NAME to $BIN_DIR/"

# Sign the binary (required on Apple Silicon to avoid CODESIGNING crashes)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Signing binary..."
    codesign -s - "../$BIN_DIR/$BINARY_NAME"
fi

echo "Done."

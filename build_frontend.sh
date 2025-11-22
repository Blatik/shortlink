#!/bin/bash
set -e

echo "Building Frontend..."
cd frontend
# Ensure target is added
rustup target add wasm32-unknown-unknown

# Install trunk if not present (optional, user might have it)
if ! command -v trunk &> /dev/null; then
    echo "Trunk not found. Installing..."
    cargo install trunk
fi

# Build release
trunk build --release

echo "Build complete! Output in ../docs"

#!/bin/bash

# Desktop Build Script
# This script builds the desktop package in release mode.

# Navigate to the desktop package directory
cd packages/desktop

echo "🚀 Building Desktop App..."
dx build --platform desktop --release
echo "✅ Build complete! App located at: target/dx/pli_dxliter/release/macos/PliDxliter.app"

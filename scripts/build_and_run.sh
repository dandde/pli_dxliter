#!/bin/bash

# Sanskrit/Pali Transliteration Build & Run Script
# This script builds the web package and opens it in the browser.

# Navigate to the web package directory
cd packages/web

echo "🚀 Starting Dioxus Web Server..."
# dx serve --open will build and open the browser automatically
dx serve --open

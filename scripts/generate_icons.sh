#!/bin/bash
set -e

SOURCE="packages/desktop/assets/icon_original.png"
DEST_DIR="packages/desktop/assets/icon.iconset"
FINAL_ICON="packages/desktop/assets/icon.icns"

mkdir -p "$DEST_DIR"

sips -z 16 16     "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_16x16.png"
sips -z 32 32     "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_16x16@2x.png"
sips -z 32 32     "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_32x32.png"
sips -z 64 64     "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_32x32@2x.png"
sips -z 128 128   "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_128x128.png"
sips -z 256 256   "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_128x128@2x.png"
sips -z 256 256   "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_256x256.png"
sips -z 512 512   "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_256x256@2x.png"
sips -z 512 512   "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_512x512.png"
sips -z 1024 1024 "$SOURCE" --setProperty format png --out "$DEST_DIR/icon_512x512@2x.png"

iconutil -c icns "$DEST_DIR" -o "$FINAL_ICON"

rm -rf "$DEST_DIR"
echo "✅ Generated $FINAL_ICON"

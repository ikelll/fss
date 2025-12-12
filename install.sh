#!/usr/bin/env bash
set -e

REPO="ikelll/fss"
BINARY="fss"
INSTALL_DIR="/usr/bin"

echo "Installing latest release of $BINARY from $REPO..."

for cmd in curl unzip; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: $cmd is required"
    exit 1
  fi
done

ASSET_URL=$(
  curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
    | grep browser_download_url \
    | grep linux \
    | grep glibc \
    | grep zip \
    | cut -d '"' -f 4 \
    | head -n 1
)

if [ -z "$ASSET_URL" ]; then
  echo "Error: could not find Linux ZIP asset in latest release"
  exit 1
fi

echo "Found release asset:"
echo "  $ASSET_URL"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

ZIP_FILE="$TMP_DIR/release.zip"

echo "Downloading..."
curl -fL "$ASSET_URL" -o "$ZIP_FILE"

echo "Extracting..."
unzip -q "$ZIP_FILE" -d "$TMP_DIR"

if [ ! -f "$TMP_DIR/$BINARY" ]; then
  echo "Error: binary '$BINARY' not found in archive"
  exit 1
fi

chmod +x "$TMP_DIR/$BINARY"

echo "Installing to $INSTALL_DIR (requires sudo)..."
sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"

echo
echo "$BINARY installed successfully"
echo "Run:"
echo "$BINARY --help"

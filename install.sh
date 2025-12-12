#!/usr/bin/env bash
set -e

REPO="ikelll/fss"
BINARY="fss"
INSTALL_DIR="/usr/local/bin"

echo "Installing latest release of $BINARY from $REPO..."

command -v curl >/dev/null 2>&1 || {
  echo "Error: curl is required"
  exit 1
}

command -v tar >/dev/null 2>&1 || {
  echo "Error: tar is required"
  exit 1
}

LATEST_URL=$(
  curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
  | grep browser_download_url \
  | grep linux \
  | grep "$BINARY" \
  | cut -d '"' -f 4 \
  | head -n 1
)

if [ -z "$LATEST_URL" ]; then
  echo "Error: could not find Linux binary in latest release"
  exit 1
fi

echo "Found release asset:"
echo "  $LATEST_URL"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading..."
curl -fL "$LATEST_URL" -o "$TMP_DIR/$BINARY"

chmod +x "$TMP_DIR/$BINARY"

echo "Installing to $INSTALL_DIR (requires sudo)..."
sudo mv "$TMP_DIR/$BINARY" "$INSTALL_DIR/$BINARY"

echo
echo "$BINARY installed successfully"
echo "Run:"
echo "  $BINARY --help"

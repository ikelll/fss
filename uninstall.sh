#!/usr/bin/env bash
set -e

BINARY="fss"
INSTALL_DIR="/usr/bin"
TARGET="$INSTALL_DIR/$BINARY"

echo "Uninstalling $BINARY..."

if [ ! -f "$TARGET" ]; then
  echo "✔ $BINARY is not installed in $INSTALL_DIR"
  exit 0
fi

echo "Found $TARGET"

echo "Removing (requires sudo)..."
sudo rm -f "$TARGET"

echo "$BINARY removed successfully"

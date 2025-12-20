#!/usr/bin/env bash
# Улучшенный установщик monarch v1.0.0 (vladislav-yemelyanov/monarch)
# Работает на macOS, Linux, arm64/aarch64, x86_64
set -euo pipefail
IFS=$'\n\t'

REPO="vladislav-yemelyanov/monarch"
BINARY="monarch"
VERSION="1.0.0"
INSTALL_DIR="/usr/local/bin"

# Определяем ОС и архитектуру
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)  TARGET_OS="apple-darwin" ;;
  Linux)   TARGET_OS="unknown-linux-gnu" ;;
  *)       echo "❌ Unsupported OS: $OS" >&2; exit 1 ;;
esac

case "$ARCH" in
  aarch64|arm64)   TARGET_ARCH="aarch64" ;;
  x86_64|amd64)    TARGET_ARCH="x86_64" ;;
  *)               echo "❌ Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

# Формируем имя архива и URL
ARCHIVE="${BINARY}-v${VERSION}-${TARGET_ARCH}-${TARGET_OS}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading monarch v${VERSION} for ${TARGET_ARCH}-${TARGET_OS}..."
curl --fail --location --progress-bar --output "$TMP_DIR/$ARCHIVE" "$URL" || {
  echo "❌ Download failed. Check if release exists: $URL" >&2
  exit 1
}

echo "Extracting..."
tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR" || {
  echo "❌ Failed to extract archive" >&2
  exit 1
}

# Ищем бинарник в распакованной структуре
BINARY_PATH=""
if [[ -f "$TMP_DIR/$BINARY" ]]; then
  BINARY_PATH="$TMP_DIR/$BINARY"
elif [[ -f "$TMP_DIR/tmp/$BINARY" ]]; then
  BINARY_PATH="$TMP_DIR/tmp/$BINARY"
else
  # Ищем рекурсивно, если структура неожиданная
  BINARY_PATH="$(find "$TMP_DIR" -type f -name "$BINARY" | head -n 1)"
fi

if [[ -z "$BINARY_PATH" || ! -f "$BINARY_PATH" ]]; then
  echo "❌ Binary not found in archive" >&2
  echo "Archive contents:" >&2
  ls -lR "$TMP_DIR" >&2
  exit 1
fi

chmod +x "$BINARY_PATH"

# Установка в /usr/local/bin (с sudo, если нужно)
echo "Installing to $INSTALL_DIR (may require password)..."
if [[ -w "$INSTALL_DIR" ]]; then
  mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY"
else
  sudo mv "$BINARY_PATH" "$INSTALL_DIR/$BINARY"
fi

# Проверка, что monarch работает
if command -v "$BINARY" >/dev/null; then
  echo "✅ monarch v${VERSION} installed successfully!"
  echo
  echo "Run: $BINARY --help"
  echo "Version: $($BINARY --version 2>/dev/null || echo 'v1.0.0')"
else
  echo "⚠️ Installation successful, but command not found in PATH"
  echo "Add to PATH: export PATH=\"\$PATH:$INSTALL_DIR\""
fi

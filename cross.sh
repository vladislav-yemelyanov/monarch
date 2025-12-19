#!/usr/bin/env bash
set -e

VERSION="v1.0.0"
BIN="monarch"
RELEASE_DIR="releases/$VERSION"
TARGETS=("x86_64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin")

mkdir -p "$RELEASE_DIR"

for TARGET in "${TARGETS[@]}"; do
  if [ "$TARGET" = "x86_64-unknown-linux-gnu" ]; then
    cross build --release --target "$TARGET"
  else
    cargo build --release --target "$TARGET"
  fi

  TMP_DIR="$RELEASE_DIR/tmp"
  mkdir -p "$TMP_DIR"
  cp "target/$TARGET/release/$BIN" "$TMP_DIR/"
  cp README.md LICENSE "$TMP_DIR/" || true
  tar -czf "$RELEASE_DIR/$BIN-$VERSION-$TARGET.tar.gz" -C "$RELEASE_DIR" tmp
  rm -rf "$TMP_DIR"
done

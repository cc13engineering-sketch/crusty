#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
ENGINE="$ROOT/engine"
PKG="$ROOT/_pkg"
SITE_SRC="$ROOT/site"
SITE_OUT="$ROOT/_site"

echo "==> Building WASM (no toml-presets for smaller binary)..."
cd "$ENGINE"
wasm-pack build crates/engine-core --target web --out-dir "$PKG" \
  -- --no-default-features

echo "==> Assembling site..."
mkdir -p "$SITE_OUT/pkg"

# Copy site sources
cp -r "$SITE_SRC"/* "$SITE_OUT/"

# Copy WASM artifacts
cp "$PKG/engine_core.js" "$SITE_OUT/pkg/"
cp "$PKG/engine_core_bg.wasm" "$SITE_OUT/pkg/"

# Cache-bust: stamp HTML files with a hash of the WASM binary
WASM_HASH=$(sha256sum "$PKG/engine_core_bg.wasm" | cut -c1-12)
echo "==> WASM hash: $WASM_HASH"
find "$SITE_OUT" -name '*.html' -exec sed -i "s/__WASM_HASH__/$WASM_HASH/g" {} +

# Precompress for servers that support it (nginx gzip_static, etc.)
if command -v gzip >/dev/null 2>&1; then
    gzip -9 -k -f "$SITE_OUT/pkg/engine_core_bg.wasm"
    echo "==> Precompressed: $(ls -lh "$SITE_OUT/pkg/engine_core_bg.wasm.gz" | awk '{print $5}') gzipped"
fi

RAW_SIZE=$(ls -lh "$SITE_OUT/pkg/engine_core_bg.wasm" | awk '{print $5}')
echo "==> Done. Site ready at $SITE_OUT/ (WASM: $RAW_SIZE raw)"

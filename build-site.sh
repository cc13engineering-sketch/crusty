#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
ENGINE="$ROOT/engine"
PKG="$ROOT/_pkg"
SITE_SRC="$ROOT/site"
SITE_OUT="$ROOT/_site"

echo "==> Building WASM..."
cd "$ENGINE"
wasm-pack build crates/engine-core --target web --out-dir "$PKG"

echo "==> Assembling site..."
mkdir -p "$SITE_OUT/pkg"

# Copy site sources
cp -r "$SITE_SRC"/* "$SITE_OUT/"

# Copy WASM artifacts
cp "$PKG/engine_core.js" "$SITE_OUT/pkg/"
cp "$PKG/engine_core_bg.wasm" "$SITE_OUT/pkg/"

echo "==> Done. Site ready at $SITE_OUT/"

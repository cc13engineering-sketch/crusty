#!/usr/bin/env bash
set -euo pipefail

# ── Gravity Pong local dev server with live reload ──────────────────────
# Usage: ./dev.sh [--port 3000]
#
# Builds WASM, assembles site, serves on localhost, and watches for
# Rust/HTML changes to rebuild + reload the browser automatically.

PORT="${1:-3000}"
if [ "$1" = "--port" ] 2>/dev/null; then PORT="${2:-3000}"; fi

ROOT="$(cd "$(dirname "$0")" && pwd)"
ENGINE="$ROOT/engine"
PKG="$ROOT/_pkg"
SITE_SRC="$ROOT/site"
SITE_OUT="$ROOT/_site"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

cleanup() {
    echo -e "\n${YELLOW}Shutting down...${NC}"
    kill "$SERVER_PID" 2>/dev/null || true
    kill "$WATCH_PID" 2>/dev/null || true
    exit 0
}
trap cleanup INT TERM

# ── Dependency check ────────────────────────────────────────────────────

check_dep() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo -e "${RED}Missing: $1${NC} — $2"
        return 1
    fi
}

MISSING=0
check_dep wasm-pack "cargo install wasm-pack" || MISSING=1
check_dep python3 "brew install python3 / apt install python3" || MISSING=1

if [ "$MISSING" -eq 1 ]; then
    echo -e "${RED}Install missing dependencies and retry.${NC}"
    exit 1
fi

# ── Build function ──────────────────────────────────────────────────────

build() {
    local START=$(date +%s%N 2>/dev/null || python3 -c 'import time; print(int(time.time()*1e9))')

    echo -e "${CYAN}Building WASM...${NC}"
    cd "$ENGINE"
    if ! wasm-pack build crates/engine-core --target web --dev --out-dir "$PKG" \
        -- --no-default-features 2>&1 | tail -5; then
        echo -e "${RED}Build failed!${NC}"
        return 1
    fi

    echo -e "${CYAN}Assembling site...${NC}"
    mkdir -p "$SITE_OUT/pkg"
    cp -r "$SITE_SRC"/* "$SITE_OUT/"
    cp "$PKG/engine_core.js" "$SITE_OUT/pkg/"
    cp "$PKG/engine_core_bg.wasm" "$SITE_OUT/pkg/"

    # Inject live-reload snippet into HTML (replaces cache-bust placeholder)
    find "$SITE_OUT" -name '*.html' -exec sed -i '' "s/__WASM_HASH__/dev/g" {} +

    # Inject live-reload script before </body>
    local RELOAD_SCRIPT='<script>(() => { let last = 0; setInterval(async () => { try { const r = await fetch("/__reload"); const t = await r.text(); if (last \&\& t !== last) location.reload(); last = t; } catch(e) {} }, 800); })();</script>'
    find "$SITE_OUT" -name '*.html' -exec sed -i '' "s|</body>|${RELOAD_SCRIPT}</body>|g" {} +

    local END=$(date +%s%N 2>/dev/null || python3 -c 'import time; print(int(time.time()*1e9))')
    local MS=$(( (END - START) / 1000000 ))
    echo -e "${GREEN}Build complete in ${MS}ms${NC}"

    # Touch reload marker
    echo "$(date +%s)" > "$SITE_OUT/__reload_stamp"
}

# ── Tiny HTTP server with reload endpoint ───────────────────────────────

start_server() {
    python3 -c "
import http.server, os, sys

SITE = '$SITE_OUT'

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **kw):
        super().__init__(*a, directory=SITE, **kw)

    def do_GET(self):
        if self.path == '/__reload':
            stamp_path = os.path.join(SITE, '__reload_stamp')
            try:
                with open(stamp_path) as f:
                    stamp = f.read().strip()
            except FileNotFoundError:
                stamp = '0'
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain')
            self.send_header('Cache-Control', 'no-cache')
            self.end_headers()
            self.wfile.write(stamp.encode())
            return
        # Serve .wasm with correct MIME type
        if self.path.endswith('.wasm'):
            self.extensions_map['.wasm'] = 'application/wasm'
        return super().do_GET()

    def log_message(self, fmt, *args):
        # Suppress reload polling noise
        if '/__reload' in (args[0] if args else ''):
            return
        super().log_message(fmt, *args)

    def end_headers(self):
        self.send_header('Cache-Control', 'no-cache, no-store')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        super().end_headers()

s = http.server.HTTPServer(('0.0.0.0', $PORT), Handler)
print(f'Serving on http://localhost:$PORT', flush=True)
s.serve_forever()
" &
    SERVER_PID=$!
}

# ── File watcher ────────────────────────────────────────────────────────

watch_and_rebuild() {
    # Use fswatch if available, otherwise fall back to polling
    if command -v fswatch >/dev/null 2>&1; then
        echo -e "${CYAN}Watching for changes (fswatch)...${NC}"
        fswatch -r -l 1 \
            --include='\.rs$' --include='\.html$' --include='\.css$' --include='\.js$' \
            --exclude='.*' \
            "$ENGINE/crates/engine-core/src" "$SITE_SRC" | while read -r _; do
            # Drain extra events from batch
            while read -r -t 0.5 _; do :; done
            echo -e "\n${YELLOW}Change detected, rebuilding...${NC}"
            build
        done
    else
        echo -e "${CYAN}Watching for changes (polling, install fswatch for faster reloads)...${NC}"
        local LAST_HASH=""
        while true; do
            sleep 2
            # Hash mtimes of source files
            local HASH
            HASH=$(find "$ENGINE/crates/engine-core/src" "$SITE_SRC" \
                -type f \( -name '*.rs' -o -name '*.html' -o -name '*.css' -o -name '*.js' \) \
                -newer "$SITE_OUT/__reload_stamp" 2>/dev/null | head -1)
            if [ -n "$HASH" ]; then
                echo -e "\n${YELLOW}Change detected, rebuilding...${NC}"
                build
            fi
        done
    fi
}

# ── Main ────────────────────────────────────────────────────────────────

echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   Gravity Pong Dev Server            ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"

echo -e "${CYAN}Initial build...${NC}"
build

start_server
echo -e "\n${GREEN}→ http://localhost:${PORT}/gravity-pong/${NC}\n"

watch_and_rebuild &
WATCH_PID=$!

wait

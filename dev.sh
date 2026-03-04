#!/usr/bin/env bash
set -euo pipefail

# ── Gravity Pong local dev server with live reload ──────────────────────
# Usage: ./dev.sh [PORT]         (default 3000)

PORT="${1:-3000}"

ROOT="$(cd "$(dirname "$0")" && pwd)"
ENGINE="$ROOT/engine"
PKG="$ROOT/_pkg"
SITE_SRC="$ROOT/site"
SITE_OUT="$ROOT/_site"

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

# ── Build ───────────────────────────────────────────────────────────────

build() {
    echo -e "${CYAN}Building WASM...${NC}"
    cd "$ENGINE"
    if ! wasm-pack build crates/engine-core --target web --release --out-dir "$PKG" \
        -- --no-default-features 2>&1 | tail -5; then
        echo -e "${RED}Build failed!${NC}"
        return 1
    fi

    echo -e "${CYAN}Assembling site...${NC}"
    rm -rf "$SITE_OUT"
    mkdir -p "$SITE_OUT/pkg"
    cp -r "$SITE_SRC"/* "$SITE_OUT/"
    cp "$PKG/engine_core.js" "$SITE_OUT/pkg/"
    cp "$PKG/engine_core_bg.wasm" "$SITE_OUT/pkg/"

    # Replace cache-bust placeholder
    find "$SITE_OUT" -name '*.html' -exec sed -i '' 's/__WASM_HASH__/dev/g' {} +

    # Inject live-reload script
    local RELOAD='<script>(()=>{let l=0;setInterval(async()=>{try{const r=await fetch("/__reload");const t=await r.text();if(l\&\&t!==l)location.reload();l=t}catch(e){}},800)})()</script>'
    find "$SITE_OUT" -name '*.html' -exec sed -i '' "s|</body>|${RELOAD}</body>|g" {} +

    # Reload stamp
    date +%s > "$SITE_OUT/__reload_stamp"

    echo -e "${GREEN}Build complete${NC}"
}

# ── HTTP server ─────────────────────────────────────────────────────────

start_server() {
    python3 << 'PYEOF' &
import http.server, os

SITE = os.environ["SITE_OUT"]
PORT = int(os.environ["DEV_PORT"])

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *a, **kw):
        super().__init__(*a, directory=SITE, **kw)

    def do_GET(self):
        if self.path == "/__reload":
            stamp_path = os.path.join(SITE, "__reload_stamp")
            try:
                with open(stamp_path) as f:
                    stamp = f.read().strip()
            except FileNotFoundError:
                stamp = "0"
            self.send_response(200)
            self.send_header("Content-Type", "text/plain")
            self.send_header("Cache-Control", "no-cache")
            self.end_headers()
            self.wfile.write(stamp.encode())
            return
        return super().do_GET()

    def log_message(self, fmt, *args):
        msg = str(args[0]) if args else ""
        if "/__reload" in msg or "favicon" in msg:
            return
        super().log_message(fmt, *args)

    def end_headers(self):
        self.send_header("Cache-Control", "no-cache, no-store")
        super().end_headers()

s = http.server.HTTPServer(("0.0.0.0", PORT), Handler)
print(f"Serving on http://localhost:{PORT}", flush=True)
s.serve_forever()
PYEOF
    SERVER_PID=$!
}

# ── File watcher ────────────────────────────────────────────────────────

watch_and_rebuild() {
    if command -v fswatch >/dev/null 2>&1; then
        echo -e "${CYAN}Watching for changes (fswatch)...${NC}"
        fswatch -r -l 1 \
            --include='\.rs$' --include='\.html$' --include='\.css$' --include='\.js$' \
            --exclude='.*' \
            "$ENGINE/crates/engine-core/src" "$SITE_SRC" | while read -r _; do
            while read -r -t 0.5 _; do :; done
            echo -e "\n${YELLOW}Change detected, rebuilding...${NC}"
            build
        done
    else
        echo -e "${CYAN}Watching for changes (polling every 2s)...${NC}"
        while true; do
            sleep 2
            CHANGED=$(find "$ENGINE/crates/engine-core/src" "$SITE_SRC" \
                -type f \( -name '*.rs' -o -name '*.html' -o -name '*.css' -o -name '*.js' \) \
                -newer "$SITE_OUT/__reload_stamp" 2>/dev/null | head -1)
            if [ -n "$CHANGED" ]; then
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

build

export SITE_OUT DEV_PORT="$PORT"
start_server
echo -e "\n${GREEN}→ http://localhost:${PORT}/gravity-pong/${NC}\n"

watch_and_rebuild &
WATCH_PID=$!

wait

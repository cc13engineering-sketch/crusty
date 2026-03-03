#!/bin/bash
# RNG lint — fail if forbidden RNG patterns appear in engine-core.
#
# Allowed RNG:
#   - SeededRng (in rng.rs) — the engine's canonical PRNG
#   - LcgRng (in headless/action_gen.rs) — test infrastructure, intentionally separate
#   - RandomPolicy (in policy.rs) — uses its own LCG, separate from engine RNG
#
# Forbidden:
#   - thread_rng, rand::random — non-deterministic
#   - OsRng, StdRng — non-deterministic
#   - sin()-based RNG patterns — ad-hoc, non-reproducible
#   - SimpleRng — deleted, replaced by SeededRng

set -e

SRC_DIR="$(dirname "$0")/../crates/engine-core/src"

FORBIDDEN_PATTERNS=(
    "thread_rng"
    "rand::random"
    "OsRng"
    "StdRng"
    "SimpleRng"
)

FOUND=0

for pattern in "${FORBIDDEN_PATTERNS[@]}"; do
    matches=$(grep -rn "$pattern" "$SRC_DIR" --include="*.rs" 2>/dev/null || true)
    if [ -n "$matches" ]; then
        echo "FAIL: forbidden RNG pattern '$pattern' found:"
        echo "$matches" | sed 's/^/  /'
        FOUND=1
    fi
done

if [ $FOUND -eq 0 ]; then
    echo "PASS: no forbidden RNG patterns found in engine-core"
    exit 0
else
    echo ""
    echo "Use SeededRng (from rng.rs) for all engine RNG needs."
    echo "Policy/test RNG must be kept separate from engine RNG."
    exit 1
fi

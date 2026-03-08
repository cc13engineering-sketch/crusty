#!/usr/bin/env bash
#
# reorder.sh — Sort all_sprints.jsonl by sprint_index (ascending, numeric)
#
# Reads all_sprints.jsonl, sorts by the sprint_index field, writes
# the result to all_sprints_ordered.jsonl, and verifies correctness.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INPUT="$SCRIPT_DIR/all_sprints.jsonl"
OUTPUT="$SCRIPT_DIR/all_sprints_ordered.jsonl"

if [[ ! -f "$INPUT" ]]; then
    echo "ERROR: Input file not found: $INPUT"
    exit 1
fi

# Sort by sprint_index using jq to extract the key, then decorate-sort-undecorate
# 1. Prepend each line with its sprint_index (tab-separated)
# 2. Sort numerically by that field
# 3. Strip the prefix, leaving just the sorted JSON lines
jq -r '"\(.sprint_index)\t\(. | @json)"' "$INPUT" \
    | sort -t$'\t' -k1,1n \
    | cut -f2- \
    | jq -c '.' \
    > "$OUTPUT"

# --- Summary ---
TOTAL=$(wc -l < "$OUTPUT" | tr -d ' ')
FIRST=$(head -1 "$OUTPUT" | jq '.sprint_index')
LAST=$(tail -1 "$OUTPUT" | jq '.sprint_index')

echo "=== Reorder Summary ==="
echo "Total lines:        $TOTAL"
echo "First sprint_index: $FIRST"
echo "Last sprint_index:  $LAST"

# --- Verification: check for duplicates and gaps in 1..264 ---
INDICES=$(jq '.sprint_index' "$OUTPUT" | sort -n)
UNIQUE_COUNT=$(echo "$INDICES" | sort -nu | wc -l | tr -d ' ')
EXPECTED=264

echo ""
echo "=== Verification ==="
echo "Unique indices:     $UNIQUE_COUNT"
echo "Expected indices:   $EXPECTED"

if [[ "$TOTAL" -ne "$EXPECTED" ]]; then
    echo "WARNING: Line count ($TOTAL) does not match expected ($EXPECTED)"
fi

if [[ "$UNIQUE_COUNT" -ne "$EXPECTED" ]]; then
    echo "WARNING: Unique index count ($UNIQUE_COUNT) does not match expected ($EXPECTED)"
fi

# Check for duplicates
DUPES=$(echo "$INDICES" | uniq -d)
if [[ -n "$DUPES" ]]; then
    echo "DUPLICATES found: $DUPES"
else
    echo "No duplicate sprint indices."
fi

# Check for missing indices in range 1..264
MISSING=$(comm -23 <(seq 1 "$EXPECTED") <(echo "$INDICES" | sort -n))
if [[ -n "$MISSING" ]]; then
    echo "MISSING indices: $MISSING"
else
    echo "No missing sprint indices (1-$EXPECTED all present)."
fi

echo ""
echo "Output written to: $OUTPUT"

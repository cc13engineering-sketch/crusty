#!/usr/bin/env bash
# Build HTML docs from markdown files in docs/ -> site/docs/
# Uses a simple sed-based approach that works without external dependencies.
# For CI, we use Python's markdown module if available, otherwise a basic converter.

set -euo pipefail

DOCS_DIR="$(cd "$(dirname "$0")/.." && pwd)/docs"
OUT_DIR="$(cd "$(dirname "$0")/.." && pwd)/site/docs"

mkdir -p "$OUT_DIR"

# HTML template wrapper
html_head() {
    local title="$1"
    cat <<HEADER
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${title} — Crusty Engine</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #0d0d14;
            color: #d0d0d0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace;
            max-width: 860px;
            margin: 0 auto;
            padding: 40px 24px;
            line-height: 1.7;
        }
        h1 { color: #f0e060; margin-bottom: 8px; font-size: 1.8rem; }
        h2 { color: #f0e060; margin-top: 32px; margin-bottom: 12px; font-size: 1.3rem; border-bottom: 1px solid #2a2a3a; padding-bottom: 6px; }
        h3 { color: #e0d050; margin-top: 24px; margin-bottom: 8px; font-size: 1.1rem; }
        a { color: #f0e060; }
        code { background: #1a1a28; padding: 2px 6px; border-radius: 3px; font-size: 0.9em; }
        pre { background: #1a1a28; padding: 16px; border-radius: 6px; overflow-x: auto; margin: 12px 0; border: 1px solid #2a2a3a; }
        pre code { background: none; padding: 0; }
        table { border-collapse: collapse; width: 100%; margin: 12px 0; }
        th, td { border: 1px solid #2a2a3a; padding: 8px 12px; text-align: left; }
        th { background: #1a1a28; color: #f0e060; }
        ul, ol { margin: 8px 0 8px 24px; }
        li { margin: 4px 0; }
        blockquote { border-left: 3px solid #f0e060; padding-left: 16px; color: #999; margin: 12px 0; }
        p { margin: 8px 0; }
        strong { color: #e0e0e0; }
        .nav { margin-bottom: 32px; }
        .nav a { margin-right: 16px; font-size: 0.9rem; }
        hr { border: none; border-top: 1px solid #2a2a3a; margin: 24px 0; }
    </style>
</head>
<body>
<nav class="nav">
    <a href="../">&larr; Home</a>
    <a href="./">Docs</a>
    <a href="engine.html">Engine</a>
    <a href="architecture.html">Architecture</a>
    <a href="api-reference.html">API</a>
    <a href="getting-started.html">Getting Started</a>
    <a href="ai-iteration.html">AI Iteration</a>
    <a href="codebase-explainer.html">Codebase Explainer</a>
</nav>
HEADER
}

html_tail() {
    cat <<FOOTER
</body>
</html>
FOOTER
}

# Convert markdown to HTML using Python if available, otherwise basic sed
convert_md() {
    local input="$1"
    if command -v python3 &>/dev/null; then
        python3 -c "
import sys, html, re

content = open('$input').read()
lines = content.split('\n')
out = []
in_code = False
in_table = False
in_list = False
code_lang = ''

for line in lines:
    # Code blocks
    if line.startswith('\`\`\`'):
        if in_code:
            out.append('</code></pre>')
            in_code = False
        else:
            code_lang = line[3:].strip()
            out.append('<pre><code>')
            in_code = True
        continue
    if in_code:
        out.append(html.escape(line))
        continue

    # Close list if empty line
    if not line.strip() and in_list:
        out.append('</ul>')
        in_list = False

    # Close table if non-table line
    if in_table and not line.strip().startswith('|'):
        out.append('</table>')
        in_table = False

    # Headers
    if line.startswith('# '):
        out.append(f'<h1>{html.escape(line[2:])}</h1>')
    elif line.startswith('## '):
        out.append(f'<h2>{html.escape(line[3:])}</h2>')
    elif line.startswith('### '):
        out.append(f'<h3>{html.escape(line[4:])}</h3>')
    elif line.startswith('#### '):
        out.append(f'<h3>{html.escape(line[5:])}</h3>')
    # Horizontal rule
    elif line.strip() in ('---', '***', '___'):
        out.append('<hr>')
    # Table
    elif line.strip().startswith('|'):
        cells = [c.strip() for c in line.strip().strip('|').split('|')]
        if all(set(c) <= set('- :') for c in cells):
            continue  # skip separator
        if not in_table:
            out.append('<table>')
            tag = 'th'
            in_table = True
        else:
            tag = 'td'
        row = ''.join(f'<{tag}>{html.escape(c)}</{tag}>' for c in cells)
        out.append(f'<tr>{row}</tr>')
    # Unordered list
    elif line.strip().startswith('- ') or line.strip().startswith('* '):
        if not in_list:
            out.append('<ul>')
            in_list = True
        item = line.strip()[2:]
        out.append(f'<li>{html.escape(item)}</li>')
    # Ordered list
    elif re.match(r'^\d+\. ', line.strip()):
        if not in_list:
            out.append('<ul>')
            in_list = True
        item = re.sub(r'^\d+\. ', '', line.strip())
        out.append(f'<li>{html.escape(item)}</li>')
    # Empty line
    elif not line.strip():
        out.append('')
    # Regular text
    else:
        text = html.escape(line)
        out.append(f'<p>{text}</p>')

if in_list: out.append('</ul>')
if in_table: out.append('</table>')
if in_code: out.append('</code></pre>')

# Post-process: inline code and bold
result = '\n'.join(out)
result = re.sub(r'\&lt;code\&gt;(.*?)\&lt;/code\&gt;', r'<code>\1</code>', result)
result = re.sub(r'\x60([^\x60]+)\x60', r'<code>\1</code>', result)
result = re.sub(r'\*\*([^*]+)\*\*', r'<strong>\1</strong>', result)

print(result)
"
    else
        # Fallback: just wrap in <pre>
        echo "<pre>"
        cat "$input" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g'
        echo "</pre>"
    fi
}

# Extract title from first H1 in markdown
get_title() {
    head -5 "$1" | grep '^# ' | head -1 | sed 's/^# //'
}

# Build each doc
for md in "$DOCS_DIR"/*.md; do
    base=$(basename "$md" .md)
    title=$(get_title "$md")
    [ -z "$title" ] && title="$base"
    out_file="$OUT_DIR/${base}.html"

    echo "  $base.md -> $base.html"
    {
        html_head "$title"
        convert_md "$md"
        html_tail
    } > "$out_file"
done

# Build index page
echo "  index.html"
cat > "$OUT_DIR/index.html" <<'INDEX'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentation — Crusty Engine</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            background: #0d0d14;
            color: #d0d0d0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, monospace;
            max-width: 860px;
            margin: 0 auto;
            padding: 40px 24px;
            line-height: 1.7;
        }
        h1 { color: #f0e060; margin-bottom: 24px; }
        a { color: #f0e060; }
        .doc-list { list-style: none; padding: 0; }
        .doc-list li {
            margin: 12px 0;
            padding: 16px 20px;
            background: #1a1a28;
            border: 1px solid #2a2a3a;
            border-radius: 6px;
        }
        .doc-list li:hover { border-color: #f0e060; }
        .doc-list a { text-decoration: none; font-size: 1.1rem; }
        .doc-list p { color: #888; font-size: 0.9rem; margin-top: 4px; }
        .nav { margin-bottom: 32px; }
        .nav a { margin-right: 16px; font-size: 0.9rem; }
    </style>
</head>
<body>
<nav class="nav"><a href="../">&larr; Home</a></nav>
<h1>Crusty Engine Documentation</h1>
<ul class="doc-list">
    <li><a href="getting-started.html">Getting Started</a><p>From zero to running simulations and headless analysis.</p></li>
    <li><a href="engine.html">Engine Architecture</a><p>ECS, 5-phase tick loop, rendering, physics, determinism.</p></li>
    <li><a href="architecture.html">Headless Testing Architecture</a><p>22-module infrastructure for AI-driven game testing.</p></li>
    <li><a href="api-reference.html">API Reference</a><p>Core types, headless modules, CLI commands.</p></li>
    <li><a href="ai-iteration.html">AI Iteration Guide</a><p>How AI agents use the engine to build and test games.</p></li>
    <li><a href="codebase-explainer.html">Codebase Explainer</a><p>Complete walkthrough of the engine for senior TypeScript engineers new to Rust and game dev.</p></li>
</ul>
</body>
</html>
INDEX

echo "Done. Generated $(ls "$OUT_DIR"/*.html | wc -l) HTML files in site/docs/"

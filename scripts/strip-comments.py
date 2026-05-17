#!/usr/bin/env python3
"""Strip all comments from source files (Rust, TS/JS, Svelte, CSS)."""
import re
import sys
from pathlib import Path


IDENT = set('abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_')


def strip_c_style(src: str, is_rust: bool = False) -> str:
    """Strip // and /* */ comments, respecting string/char literals.

    Handles: " ", ' ', `` ` `` (template strings), Rust raw strings r#"..."#,
    Rust lifetimes ('a, 'static, '_) vs char literals ('x', '\n').
    """
    out = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        nxt = src[i + 1] if i + 1 < n else ''
        prev = src[i - 1] if i > 0 else ''
        if is_rust and c == 'r' and nxt in ('#', '"') and prev not in IDENT:
            hash_count = 0
            j = i + 1
            while j < n and src[j] == '#':
                hash_count += 1
                j += 1
            if j < n and src[j] == '"':
                end_marker = '"' + '#' * hash_count
                close = src.find(end_marker, j + 1)
                if close == -1:
                    out.append(src[i:])
                    return ''.join(out)
                out.append(src[i:close + len(end_marker)])
                i = close + len(end_marker)
                continue
        if is_rust and c == "'":
            j = i + 1
            if j < n and src[j] == '\\':
                k = j + 1
                while k < n and src[k] != "'":
                    k += 1
                if k < n:
                    out.append(src[i:k + 1])
                    i = k + 1
                    continue
            if j + 1 < n and src[j + 1] == "'":
                out.append(src[i:j + 2])
                i = j + 2
                continue
            out.append(c)
            i += 1
            continue
        if c == '"' or (not is_rust and c == "'") or c == '`':
            out.append(c)
            j = i + 1
            while j < n:
                cj = src[j]
                if cj == '\\' and j + 1 < n:
                    out.append(src[j:j + 2])
                    j += 2
                    continue
                out.append(cj)
                if cj == c:
                    j += 1
                    break
                j += 1
            i = j
            continue
        if c == '/' and nxt == '/':
            j = i
            while j < n and src[j] != '\n':
                j += 1
            i = j
            continue
        if c == '/' and nxt == '*':
            j = i + 2
            while j + 1 < n and not (src[j] == '*' and src[j + 1] == '/'):
                j += 1
            i = j + 2
            continue
        out.append(c)
        i += 1
    return ''.join(out)


def strip_svelte(src: str) -> str:
    """Svelte = HTML + <script> + <style>. Strip <!-- --> and C-style comments."""
    src = re.sub(r'<!--[\s\S]*?-->', '', src)
    return strip_c_style(src, is_rust=False)


def strip_rust(src: str) -> str:
    return strip_c_style(src, is_rust=True)


def strip_css(src: str) -> str:
    return re.sub(r'/\*[\s\S]*?\*/', '', src)


def collapse_blank_lines(src: str) -> str:
    """Reduce runs of >2 blank lines to a single blank line."""
    return re.sub(r'\n{3,}', '\n\n', src)


HANDLERS = {
    '.rs': strip_rust,
    '.ts': strip_c_style,
    '.tsx': strip_c_style,
    '.js': strip_c_style,
    '.mjs': strip_c_style,
    '.svelte': strip_svelte,
    '.css': strip_css,
}

ROOT = Path(__file__).resolve().parent.parent
TARGETS = [
    ROOT / 'src',
    ROOT / 'src-tauri' / 'src',
]
EXCLUDE_DIRS = {'node_modules', 'target', 'build', '.svelte-kit', 'gen'}


def should_skip(path: Path) -> bool:
    return any(part in EXCLUDE_DIRS for part in path.parts)


def main():
    total = 0
    changed = 0
    for base in TARGETS:
        for path in base.rglob('*'):
            if not path.is_file():
                continue
            if should_skip(path):
                continue
            handler = HANDLERS.get(path.suffix.lower())
            if not handler:
                continue
            total += 1
            original = path.read_text(encoding='utf-8')
            stripped = collapse_blank_lines(handler(original))
            if stripped != original:
                path.write_text(stripped, encoding='utf-8')
                changed += 1
                print(f"  stripped: {path.relative_to(ROOT)}")
    print(f"\nscanned: {total}, modified: {changed}")


if __name__ == '__main__':
    main()

#!/usr/bin/env python3
# strip.py DIR: remove comments from the Rust and shell files under DIR, keeping the first header line, SAFETY, Origin and licence lines.
import os, re, sys

KEEP = re.compile(r'^\s*//(?!/)\s*(SAFETY|Origin|SPDX|Copyright|Licen[sc]e)\b|^\s*//[/!]?\s*(SAFETY)', re.I)


def lex_rust(src):
    """Return list of (kind, text): code, line (a // comment), block."""
    out = []
    i, n = 0, len(src)
    code_start = 0

    def flush(upto):
        nonlocal code_start
        if upto > code_start:
            out.append(("code", src[code_start:upto]))
        code_start = upto

    while i < n:
        c = src[i]
        two = src[i:i + 2]
        if two == "//":
            flush(i)
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(("line", src[i:j]))
            i = code_start = j
        elif two == "/*":
            flush(i)
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth += 1; j += 2
                elif src.startswith("*/", j):
                    depth -= 1; j += 2
                else:
                    j += 1
            out.append(("block", src[i:j]))
            i = code_start = j
        elif c == '"' or (c in "br" and re.match(r'(b?r#*"|b")', src[i:i + 12] or "")) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = re.match(r'b?(r(#*))?"', src[i:])
            if m is None:
                i += 1
                continue
            if m.group(1):
                hashes = m.group(2)
                end = src.find('"' + hashes, i + m.end())
                j = n if end < 0 else end + 1 + len(hashes)
            else:
                j = i + m.end()
                while j < n and src[j] != '"':
                    j += 2 if src[j] == "\\" else 1
                j += 1
            i = j
        elif c == "'":
            m = re.match(r"'(\\.[^']*|[^\\'])'", src[i:])
            i += m.end() if m else 1
        else:
            i += 1
    flush(n)
    return out


def thrown_next(parts, idx):
    # A comment block over a line that throws a result away says why, and a test reads it.
    for kind, t in parts[idx + 1:]:
        if kind == "line":
            continue
        if kind == "code":
            code = t.split("\n")
            for ln in code[1:] if code[0].strip() == "" else code:
                if ln.strip():
                    return "let _ =" in ln or ".ok();" in ln
        return False
    return False


def strip_rust(text, why=False):
    parts = lex_rust(text)
    res = []
    first_kept = False
    header_done = False
    safety_doc = False
    for idx, (kind, t) in enumerate(parts):
        if kind == "code":
            if t.strip():
                safety_doc = False
            res.append(t)
        elif kind == "line":
            if t.strip() == "/// # Safety":
                safety_doc = True
            if safety_doc and t.lstrip().startswith("///"):
                res.append(t)
                continue
            at_start = not "".join(x for x in res if not x.startswith("//!") and x != "\x00").strip()
            keep = bool(KEEP.match(t))
            if why and not t.lstrip().startswith(("///", "//!")) and thrown_next(parts, idx):
                keep = True
            if at_start and t.startswith("//!") and not header_done:
                keep = t.strip() != "//!"
                header_done = not keep
            res.append(t if keep else "\x00")
        else:
            res.append("\x00")
    s = "".join(res)
    lines = []
    prev_comment = False
    for ln in s.split("\n"):
        if "\x00" in ln:
            ln = ln.replace("\x00", "").rstrip()
            if not ln.strip():
                if lines and lines[-1] == "" and prev_comment:
                    continue
                lines.append("")
                prev_comment = True
                continue
        prev_comment = False
        lines.append(ln)
    s = "\n".join(lines)
    s = re.sub(r"\n{3,}", "\n\n", s)
    return re.sub(r"(\n\s*#\[[^\n]*\])\n\n+(?=\s*\S)", r"\1\n", s)


def strip_shell(text):
    lines = text.split("\n")
    out, first = [], False
    for i, ln in enumerate(lines):
        s = ln.strip()
        if i == 0 and s.startswith("#!"):
            out.append(ln); continue
        if s.startswith("#"):
            if not first:
                first = True
                out.append(ln)
            elif re.match(r"#\s*(SPDX|Copyright|Licen)", s, re.I):
                out.append(ln)
            continue
        out.append(ln)
    return "\n".join(out)


def main(root):
    for d, dirs, files in os.walk(root):
        dirs[:] = [x for x in dirs if x not in (".git", "target", "vendor", "node_modules")]
        for f in files:
            p = os.path.join(d, f)
            try:
                t = open(p, encoding="utf-8").read()
            except Exception:
                continue
            if f.endswith(".rs"):
                new = strip_rust(t, why=("/src/admin/" in p or p.endswith(("voice/setup.rs", "common/charge.rs", "common/rfkill.rs"))))
            elif f.endswith(".sh") or f.endswith(".hook.chroot") or f.endswith(".hook.binary"):
                new = strip_shell(t)
            else:
                continue
            if new != t:
                open(p, "w", encoding="utf-8").write(new)



def selftest():
    a = "//! Header line.\n//!\n//! More.\n\n// plain comment\nfn a() {}\n\n#[derive(Debug)]\n// why\nstruct S;\n\n// SAFETY: fine\nunsafe { x() }\n/// licence note\nfn b() {} // trailing\nlet s = \"// not a comment\";\n"
    out = strip_rust(a)
    assert out.startswith("//! Header line.\n\nfn a()"), out
    assert "More" not in out and "plain comment" not in out and "trailing" not in out
    assert "#[derive(Debug)]\nstruct S;" in out, out
    assert "// SAFETY: fine" in out and "licence note" not in out
    assert '"// not a comment"' in out
    w = "fn f() {\n    // the file may be missing, and that is fine\n    let _ = std::fs::remove_file(p);\n}\n"
    assert "the file may be missing" in strip_rust(w, why=True) and "the file may be missing" not in strip_rust(w)
    assert "\n\n\n" not in strip_rust("a\n\n\n\nb\n")
    u = "/// Called by C.\n///\n/// # Safety\n/// Only C calls this.\nunsafe fn g() {}\n/// after\nfn h() {}\n"
    out = strip_rust(u)
    assert "/// # Safety\n/// Only C calls this.\nunsafe fn g()" in out and "Called by C" not in out and "after" not in out, out
    print("strip self-test ok")


if sys.argv[1:] == ["--selftest"]:
    selftest()
else:
    main(sys.argv[1])

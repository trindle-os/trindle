#!/usr/bin/env python3
"""Make the clean copy of Trindle that is published: .publish/, with the comments taken out.

The working tree keeps every comment; nothing here ever touches it. Each run
throws the old copy away and starts again from the last commit, so the build
folder, anything ignored and any change not yet committed never go in.

Kept in the copy:
  - one line at the top of each file saying what it does (the first sentence
    of its opening comment)
  - licence and copyright notices
  - shebangs, and what a program needs to run: a `// SAFETY:` note stays by
    every unsafe block, because Trindle's lint settings refuse to build one
    without it, and a comment the program looks for as a marker (where the
    tab's sizes begin in the stylesheet, where the website's thanks list is)

Every file is checked after stripping (Rust by building and testing the whole
copy, shell by `sh -n`, JavaScript by `node --check`, Python by compiling it);
a file that no longer passes goes back in with its comments, and is reported.

    python3 tools/site/strip.py            build and check .publish/
    python3 tools/site/strip.py --quick    build it, check all but the Rust

Python because it reads and rewrites text
files, which is short work here and long, fragile work in shell.
"""

import os
import pathlib
import re
import shutil
import subprocess
import sys
import io
import tarfile
import tokenize

ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
OUT = ROOT / ".publish"
LINE = "Comments are stripped from the published code to keep it lean."
LICENCE = re.compile(r"copyright|licen[cs]e|spdx|\bmit\b|\bgpl", re.I)

def markers():
    """Comments the program itself looks for, such as where the tab's sizes
    begin in the stylesheet: any comment that starts the way a string in the
    Rust does. Those are part of how it works, so they stay."""
    found = set()
    for rs in (ROOT / "src").rglob("*.rs"):
        found |= set(re.findall(r'"((?:/\*|<!--)[^"]*)"', rs.read_text()))
    return found


MARKERS = markers()

# Files that are text but not code: left exactly as they are.
UNTOUCHED = {".md", ".txt", ".lock", ".rgba", ".binary", ".gpg", ".svg"}
UNTOUCHED_NAMES = {"LICENSE", "favourites"}


def first_sentence(lines):
    """The opening comment, joined and cut to its first sentence."""
    text = " ".join(l.strip() for l in lines if l.strip())
    m = re.match(r"(.+?[.:;])(\s|$)", text)
    return (m.group(1) if m else text).rstrip(":;").rstrip() + ("" if (m and m.group(1).endswith(".")) else ".")


# Rust

RAW = re.compile(r'b?r(#*)"')
CHAR = re.compile(r"'(\\u\{[0-9a-fA-F]+\}|\\x[0-9a-fA-F]{2}|\\.|[^\\'\n])'")


def rust_pieces(src):
    """Splits Rust into ('code', text) and ('comment', text) pieces."""
    out, i, n, code = [], 0, len(src), []

    def flush():
        if code:
            out.append(("code", "".join(code)))
            code.clear()

    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            j = n if j < 0 else j
            flush()
            out.append(("comment", src[i:j]))
            i = j
        elif src.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if src.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif src.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            flush()
            out.append(("comment", src[i:j]))
            i = j
        elif (m := RAW.match(src, i)) and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            end = src.find('"' + m.group(1), m.end())
            end = n if end < 0 else end + 1 + len(m.group(1))
            code.append(src[i:end])
            i = end
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            code.append(src[i:j + 1])
            i = j + 1
        elif c == "'":
            m = CHAR.match(src, i)
            if m:
                code.append(m.group(0))
                i = m.end()
            else:
                code.append(c)
                i += 1
        else:
            code.append(c)
            i += 1
    flush()
    return out


def strip_rust(src):
    pieces = rust_pieces(src)
    # The opening block of comments, before any code.
    top, k = [], 0
    # A /// line documents the item below it, not the file, so it ends the block.
    while k < len(pieces) and not pieces[k][1].startswith("///") and (pieces[k][0] == "comment" or not pieces[k][1].strip()):
        if pieces[k][0] == "comment":
            top.append(pieces[k][1])
        k += 1
    header = []
    if top:
        body = [re.sub(r"^//[!/]?\s?", "", t) for t in top if t.startswith("//")]
        if any(LICENCE.search(t) for t in top):
            header = top[:]
        elif body:
            prefix = "//!" if top[0].startswith("//!") else "//"
            header = [f"{prefix} {first_sentence(body[:body.index('') if '' in body else len(body)])}"]
    text, keep_safety = [], False
    for kind, t in pieces[k:]:
        if kind == "code":
            text.append(t)
            if t.strip():
                keep_safety = False
        elif re.match(r"//\s*SAFETY:", t, re.I) or (keep_safety and t.startswith("//")):
            keep_safety = True
            text.append(t)
        elif LICENCE.search(t) and re.search(r"copyright|spdx", t, re.I):
            text.append(t)
        else:
            text.append(GONE)
    head = "\n".join(header) + ("\n\n" if header else "")
    return head + tidy("".join(text)).lstrip("\n")


# CSS and JavaScript

def strip_css(src, keep_header=True):
    out, i, n, header = [], 0, len(src), []
    while i < n:
        c = src[i]
        if src.startswith("/*", i):
            j = src.find("*/", i + 2)
            whole = src[i:n if j < 0 else j + 2]
            if any(whole.startswith(m) for m in MARKERS):
                out.append(whole)
                i = n if j < 0 else j + 2
                continue
            if not out and not header:
                header.append(f"/* {first_sentence(src[i + 2:j].split(chr(10))[:1] + [l for l in src[i + 2:j].split(chr(10))[1:]]).replace('*/', '')} */")
            i = n if j < 0 else j + 2
            out.append(GONE)
        elif c in "\"'":
            j = i + 1
            while j < n and src[j] != c:
                j += 2 if src[j] == "\\" else 1
            out.append(src[i:j + 1])
            i = j + 1
        else:
            out.append(c)
            i += 1
    body = tidy("".join(out)).lstrip("\n")
    return (header[0] + "\n\n" + body) if header and keep_header else body


REGEX_BEFORE = set("(,=:[!&|?{};+-*%<>~^")


def strip_js(src, keep_header=True):
    out, i, n, header = [], 0, len(src), []
    last = ""
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            if not "".join(out).strip() and not header and keep_header:
                header.append(src[i:j].strip())
            i = n if j < 0 else j
            out.append(GONE)
        elif src.startswith("/*", i):
            j = src.find("*/", i + 2)
            i = n if j < 0 else j + 2
            out.append(GONE)
        elif c in "\"'`":
            j = i + 1
            while j < n and src[j] != c:
                j += 2 if src[j] == "\\" else 1
            out.append(src[i:j + 1])
            i, last = j + 1, c
        elif c == "/" and (last in REGEX_BEFORE or last == "" or re.search(r"\breturn\s*$", "".join(out[-8:]))):
            j, in_class = i + 1, False
            while j < n and (src[j] != "/" or in_class) and src[j] != "\n":
                if src[j] == "\\":
                    j += 1
                elif src[j] == "[":
                    in_class = True
                elif src[j] == "]":
                    in_class = False
                j += 1
            out.append(src[i:j + 1])
            i, last = j + 1, "/"
        else:
            out.append(c)
            if not c.isspace():
                last = c
            i += 1
    return tidy("".join(out))


GONE = "\x00"


def tidy(text):
    """A line that held only a comment goes altogether; trailing spaces off,
    and never two blank lines running."""
    lines = [l.replace(GONE, "").rstrip() for l in text.split("\n") if not (GONE in l and not l.replace(GONE, "").strip())]
    out, blank = [], False
    for l in lines:
        if not l:
            if not blank:
                out.append(l)
            blank = True
        else:
            out.append(l)
            blank = False
    return "\n".join(out).strip("\n") + "\n"


def strip_html(src):
    def section(m):
        tag, attrs, body = m.group(1), m.group(2), m.group(3)
        body = strip_css(body, False) if tag == "style" else strip_js(body, False)
        return f"<{tag}{attrs}>\n{body}</{tag}>"
    src = re.sub(r"<!--(?!\s*\[if).*?-->",
                 lambda m: m.group(0) if any(m.group(0).startswith(k) for k in MARKERS) else GONE, src, flags=re.S)
    src = re.sub(r"<(style|script)([^>]*)>(.*?)</\1>",
                 lambda m: section(m) if m.group(3).strip() else m.group(0), src, flags=re.S)
    return tidy(src)


def strip_xml(src):
    return tidy(re.sub(r"<!--.*?-->", GONE, src, flags=re.S))


# Python

def strip_python(src):
    tokens = list(tokenize.generate_tokens(io.StringIO(src).readline))
    drop = set()
    for t in tokens:
        if t.type == tokenize.COMMENT:
            if t.start[0] == 1 and t.string.startswith("#!"):
                continue
            if LICENCE.search(t.string) and re.search(r"copyright|spdx", t.string, re.I):
                continue
            drop.add((t.start[0], t.start[1]))
    lines = src.split("\n")
    for (row, col) in sorted(drop, reverse=True):
        line = lines[row - 1]
        lines[row - 1] = line[:col].rstrip()
    out = [l for i, l in enumerate(lines) if l.strip() or not src.split("\n")[i].strip()]
    return tidy("\n".join(out))


# shell and configuration

def strip_hash(src, marker="#"):
    """Whole-line comments only: a comment after code on the same line stays,
    where telling it from a # inside quotes is not worth the risk."""
    lines = src.split("\n")
    out, i = [], 0
    if lines and lines[0].startswith("#!"):
        out.append(lines[0])
        i = 1
    top = []
    while i < len(lines) and lines[i].strip().startswith(marker):
        top.append(lines[i].strip()[len(marker):].strip())
        i += 1
    if top:
        joined = "\n".join(top)
        if LICENCE.search(joined) and re.search(r"copyright|spdx", joined, re.I):
            out.extend(lines[len(out):i])
        else:
            cut = top[:top.index("")] if "" in top else top
            out.append(f"{marker} {first_sentence(cut)}")
    heredoc = None
    for line in lines[i:]:
        s = line.strip()
        if heredoc:
            out.append(line)
            if s == heredoc:
                heredoc = None
            continue
        m = re.search(r"<<-?\s*['\"]?(\w+)['\"]?", line)
        if m and not s.startswith(marker):
            heredoc = m.group(1)
        if s.startswith(marker):
            continue
        out.append(line)
    return tidy("\n".join(out))


SHELLISH = {".sh", ".chroot", ".conf", ".service", ".path", ".cfg", ".desktop", ".ini", ".list",
            ".sources", ".toml", ".gitignore"}


def kind_of(path, text):
    ext = path.suffix
    if path.name in UNTOUCHED_NAMES or ext in UNTOUCHED:
        return None
    if ext == ".rs":
        return "rust"
    if ext == ".css":
        return "css"
    if ext == ".js":
        return "js"
    if ext == ".html":
        return "html"
    if ext in (".xml", ".policy"):
        return "xml"
    if ext == ".py" or text.startswith("#!/usr/bin/env python"):
        return "python"
    if ext in SHELLISH or text.startswith("#!/bin/sh") or text.startswith("#!/bin/bash"):
        return "shell"
    if ext == "" and text.lstrip().startswith("//"):
        return "apt"
    if ext == "" and text.lstrip().startswith("#"):
        return "shell"
    return None


STRIP = {"rust": strip_rust, "css": strip_css, "js": strip_js, "html": strip_html, "xml": strip_xml,
         "python": strip_python, "shell": strip_hash, "apt": lambda s: strip_hash(s, "//")}


def check(path, kind):
    """None when the file still works, otherwise why not."""
    if kind == "shell" and (path.suffix == ".sh" or path.read_text().startswith("#!/bin/")):
        r = subprocess.run(["sh", "-n", str(path)], capture_output=True, text=True)
        return r.stderr.strip() or None if r.returncode else None
    if kind == "js" and shutil.which("node"):
        r = subprocess.run(["node", "--check", str(path)], capture_output=True, text=True)
        return (r.stderr.strip().splitlines() or ["failed"])[0] if r.returncode else None
    if kind == "python":
        try:
            compile(path.read_text(), str(path), "exec")
        except SyntaxError as e:
            return str(e)
    return None


def committed(rel):
    """A file as it is in the last commit."""
    return subprocess.run(["git", "show", f"HEAD:{rel.as_posix()}"], cwd=ROOT, capture_output=True,
                          check=True).stdout


def rust_check():
    """Builds and tests the copy; the files named in any error, or None."""
    env = dict(os.environ, CARGO_TARGET_DIR=str(ROOT / "target" / "publish-check"))
    envsh = ROOT / "target" / "devlibs" / "env.sh"
    prefix = f". {envsh} && " if envsh.exists() else ""
    r = subprocess.run(["sh", "-c", prefix + "cargo build --release 2>&1 && cargo test --release 2>&1"],
                       cwd=OUT, env=env, capture_output=True, text=True)
    if r.returncode == 0:
        return None, r.stdout
    files = set(re.findall(r"-->\s+(\S+?\.rs):\d+", r.stdout)) | set(re.findall(r"panicked at (\S+?\.rs):", r.stdout))
    return files, r.stdout


def main():
    quick = "--quick" in sys.argv
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir()
    archive = subprocess.run(["git", "archive", "--format=tar", "HEAD"], cwd=ROOT, capture_output=True,
                             check=True).stdout
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(OUT, filter="tar")
        names = [m.name for m in tar.getmembers() if m.isfile()]
    copied, stripped, skipped = len(names), 0, []
    for name in names:
        dst = OUT / name
        try:
            text = dst.read_text()
        except UnicodeDecodeError:
            continue
        kind = kind_of(dst, text)
        if not kind:
            continue
        try:
            new = STRIP[kind](text)
        except Exception as e:  # a file the stripper cannot follow stays as it was
            skipped.append(f"{name}: {e}")
            continue
        if text.startswith("#!") and not new.startswith("#!"):
            new = text.split("\n", 1)[0] + "\n" + new
        dst.write_text(new)
        why = check(dst, kind)
        if why:
            dst.write_text(text)
            skipped.append(f"{name}: {why}")
        elif new != text:
            stripped += 1

    readme = OUT / "README.md"
    body = readme.read_text()
    if LINE not in body:
        # After the centred block at the top, where the text begins.
        at = body.find("</div>")
        at = body.find("\n", at) + 1 if at >= 0 else 0
        readme.write_text(f"{body[:at]}\n{LINE}\n{body[at:]}")

    gi = ROOT / ".gitignore"
    if ".publish/" not in gi.read_text().split():
        gi.write_text(gi.read_text().rstrip("\n") + "\n.publish/\n")

    if not quick:
        for _ in range(5):
            broken, log = rust_check()
            if broken is None:
                break
            if not broken:
                print(log[-3000:])
                sys.exit("The clean copy does not build or pass its tests, and no one file is to blame.")
            for name in broken:
                rel = pathlib.Path(name)
                rel = rel.relative_to(OUT) if rel.is_absolute() else rel
                (OUT / rel).write_bytes(committed(rel))
                skipped.append(f"{rel}: the build or a test needed its comments")
                stripped -= 1
        else:
            sys.exit("The clean copy still fails after putting five files back.")

    print(f"{copied} files copied to .publish/, comments taken out of {stripped}.")
    print("Checked: " + ("everything but Rust (--quick)" if quick else "Rust built and tested, scripts checked."))
    if skipped:
        print("Left with their comments:")
        for s in skipped:
            print(f"  {s}")


if __name__ == "__main__":
    main()

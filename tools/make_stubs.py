#!/usr/bin/env python3
"""Generate the learner (stub) version of the repo from the solutions version.

Rust files:
    // SOLUTION-START            -> block replaced by `todo!("...")`
    // SOLUTION-START: <expr>    -> block replaced by `<expr>`
    // SOLUTION-END
    Inside a SOLUTION block, lines starting with `//~ ` are the stub text: they
    are emitted (without the `//~ ` prefix) instead of the default todo!().
    // SOLUTION-ONLY-START ... // SOLUTION-ONLY-END   -> block removed
Markdown files:
    <!-- ANSWER-START --> ... <!-- ANSWER-END -->      -> "_Your answer here._"

Usage: tools/make_stubs.py <solutions_dir> <output_dir>
"""
import re
import shutil
import subprocess
import sys
from pathlib import Path

SKIP = {".git", "target", "tools"}
START = re.compile(r"^(\s*)// SOLUTION-START(?::\s*(.*))?\s*$")
END = re.compile(r"^\s*// SOLUTION-END\s*$")
STUB = re.compile(r"^(\s*)//~ ?(.*)$")
ONLY_START = re.compile(r"^\s*// SOLUTION-ONLY-START\s*$")
ONLY_END = re.compile(r"^\s*// SOLUTION-ONLY-END\s*$")
ANSWER = re.compile(r"<!-- ANSWER-START -->.*?<!-- ANSWER-END -->", re.S)


def strip_rust(text: str, rel: str) -> str:
    out, changed, mode, stub, default = [], False, None, [], ""
    lab = rel.split("/")[2] if rel.startswith("labs/") else rel.split("/")[1]
    for line in text.splitlines(keepends=True):
        if mode is None:
            m = START.match(line)
            if m:
                indent, expr = m.group(1), m.group(2)
                default = f'{indent}{expr or f"todo!({chr(34)}{lab}: implement me{chr(34)})"}\n'
                mode, changed, stub = "sol", True, []
                continue
            if ONLY_START.match(line):
                mode, changed = "only", True
                continue
            out.append(line)
        elif mode == "sol":
            if END.match(line):
                out.extend(stub or [default])
                mode = None
            else:
                sm = STUB.match(line)
                if sm:
                    stub.append(sm.group(1) + sm.group(2) + "\n")
        elif mode == "only" and ONLY_END.match(line):
            mode = None
    if mode is not None:
        raise SystemExit(f"unterminated solution block in {rel}")
    result = "".join(out)
    if changed and "#![allow(unused" not in result:
        result = "#![allow(unused)] // TODO: delete this line once the lab is done\n" + result
    return result


def main() -> None:
    src, dst = Path(sys.argv[1]).resolve(), Path(sys.argv[2]).resolve()
    if dst.exists():
        shutil.rmtree(dst)
    for path in src.rglob("*"):
        rel = path.relative_to(src)
        if rel.parts and rel.parts[0] in SKIP or "target" in rel.parts:
            continue
        target = dst / rel
        if path.is_dir():
            target.mkdir(parents=True, exist_ok=True)
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        if path.suffix == ".rs":
            target.write_text(strip_rust(path.read_text(), rel.as_posix()))
        elif path.suffix == ".md":
            target.write_text(ANSWER.sub("_Your answer here._", path.read_text()))
        else:
            shutil.copy2(path, target)
    # Removing blocks can leave blank-line runs rustfmt dislikes; normalise them.
    subprocess.run(["cargo", "fmt", "--all"], cwd=dst, check=True)


if __name__ == "__main__":
    main()

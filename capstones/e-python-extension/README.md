# Capstone E: Python extension with PyO3 (spec)

Rewrite a slow Python function in Rust and ship it as a `pip install`-able wheel.

## Setup

```bash
pipx install maturin        # or: uv tool install maturin
cd capstones/e-python-extension
maturin new --bindings pyo3 fastpy
cd fastpy
python -m venv .venv && source .venv/bin/activate
maturin develop --release   # builds and installs into the venv
python -c "import fastpy; print(fastpy.sum_as_string(1, 2))"
```

Keep this crate **out** of the root workspace. It has its own build via maturin; add `exclude = ["capstones/e-python-extension"]` to the root `[workspace]` section if Cargo complains.

## Requirements

1. Pick a real hotspot from your Apache Beam / Django work: parsing a log or CSV format, a scoring function over many records, tokenising text, or validating records against rules.
2. Expose it with `#[pyfunction]` and accept and return Python-native types (`Vec<String>` ↔ `list[str]`, `HashMap` ↔ `dict`).
3. Release the GIL for the heavy part (`py.allow_threads(|| ...)`, renamed `py.detach` in newer PyO3 releases; check the docs for your version) and use `rayon` inside it.
4. Map Rust errors to Python exceptions (`PyErr::new::<PyValueError, _>(...)`).
5. Tests on both sides: `cargo test` for the core logic, and `pytest` against the installed module.
6. Build wheels with `maturin build --release`; optionally publish to TestPyPI.

## Done when

- [ ] `pip install` of the built wheel works in a fresh venv
- [ ] A `pytest-benchmark` (or `timeit`) comparison against the pure-Python version is in the README
- [ ] The Python API has type hints (a `.pyi` stub file)

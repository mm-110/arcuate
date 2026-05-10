# Arcuate

Rust utility that acts as a **semantic bridge** between source code and its conceptual representation.
Scans a codebase and generates a mirror of Markdown files optimised for LLM consumption: signatures, docstrings, and links to the originals — without exposing full source unless needed.
Produces an `INDEX.md` with a full project tree and dual links (`.md` docs + source file) for every node.
Stack: Rust 2024 · `rustpython-parser` (AST) · `walkdir` (filesystem) · `anyhow` (errors).

---

## Architecture (Clean Architecture + Strategy)

```
┌─────────────────────────────────────────────┐
│  Application Layer  (Orchestrator)          │
│  FileScanner — walks filesystem, dispatches │
│  analyzers, builds ProjectLayout            │
└────────────────┬────────────────────────────┘
                 │ uses
┌────────────────▼────────────────────────────┐
│  Infrastructure Layer  (Adapters)           │
│    └── PythonParser  (rustpython-parser)    │
│        implements SourceCodeAnalyzer        │
└────────────────┬────────────────────────────┘
                 │ produces
┌────────────────▼────────────────────────────┐
│  Core Domain   (Entities + Ports)           │
│  entities/  DocumentedConstruct,            │
│             ParsedSourceFile, ProjectLayout │
│             SourceFileAnalysis              │
│  ports/     SourceCodeAnalyzer, ParserError │
└─────────────────────────────────────────────┘
```

**Dependency rule:** arrows point inward only — Domain knows nothing of Infrastructure or Application.

### Output layout

```
<output>/
├── INDEX.md          ← global tree with dual links
└── <mirror of src>   ← one .md per source file
```

---

## Conventions

### Module structure (modern Rust style)
Never use `mod.rs`. Instead, place a sibling `.rs` file next to each folder:

```
src/
├── domain.rs          ← declares submodules of domain/
└── domain/
    ├── entities.rs    ← declares submodules of entities/
    ├── ports.rs       ← declares submodules of ports/
    ├── entities/
    │   └── *.rs
    └── ports/
        └── *.rs
```

### Naming (DDD — Ubiquitous Language)
Names must speak the language of the business domain, not the implementation.
- **No technical metaphors** in type or field names: avoid `Node`, `Map`, `Tree`, `Sensor`, `Trie`, `Entity` — these describe data structures or engineering patterns, not the domain.
- **Ask "what is this in the real world?"** before naming: a `ProjectTree` is a `ProjectLayout`; a `LanguageSensor` is a `SourceCodeAnalyzer`; a `SemanticEntity` is a `DocumentedConstruct`.
- Field names must be self-explanatory without context: prefer `source_line_range` over `line_range`, `nested_constructs` over `children`.
- Avoid generic fillers: `data`, `info`, `value`, `item`, `obj`, `map`.

### Documentation style
- Each file gets a one-line `//!` module doc at the top.
- Every public `struct`, `enum`, and `trait` gets a single-line `///` above it.
- Prefer self-explanatory field and variable names over doc comments that explain them.
- No inline `//` comments unless the *why* is non-obvious (a hidden constraint, a workaround, a subtle invariant).
- Goal: code that reads itself; `cargo doc` as a light index, not a tutorial.

---

## Task Progress

- **Last task created:** 10 — CLI in main.rs
- **Last task completed:** 03 — FileScanner (application) — `new`, `scan`, `scan_dir`, `try_analyze`
- **Next task to work on:** 06 — OutputWriter trait (domain/ports) — poi 04 MarkdownWriter, 05 IndexWriter, 07 Orchestrator, 08 CLI

---

## External References

- [Notion Task Board](https://www.notion.so/Learning-Rust-351478041654802285c5f57c513e8233) — task database for this project. Naming convention: `Arcuate - [Task Title]`, Project tag = `Rust`.

---

## Keeping this file up to date

Update `CLAUDE.md` whenever:
- A new layer or module is added (update the architecture diagram and file tree)
- A new convention is adopted (naming, error handling, module style, etc.)
- A dependency is added or removed (update the stack line)
- A domain type is renamed or its shape changes significantly

Do **not** record ephemeral work (bug fixes, refactors, in-progress state) — only decisions that a future session needs to know to work correctly.

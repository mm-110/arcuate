# Arcuate

Rust utility that acts as a **semantic bridge** between source code and its conceptual representation.
Scans a codebase and generates a mirror of Markdown files optimised for LLM consumption: signatures, docstrings, and links to the originals — without exposing full source unless needed.
Produces an `INDEX.md` with a full project tree and dual links (`.md` docs + source file) for every node.
Stack: Rust 2024 · `rustpython-parser` (AST) · `walkdir` (filesystem) · `anyhow` (errors).

---

## Architecture (Clean Architecture + Strategy)

```
┌─────────────────────────────────────────────┐
│  Delivery Layer                             │
│  main.rs — CLI args, exit codes             │
│  delivery/factories.rs — wires adapters     │
│    into use cases                           │
└────────────────┬────────────────────────────┘
                 │ uses
┌────────────────▼────────────────────────────┐
│  Application Layer  (Use Cases)             │
│  FileScanner — walks filesystem, dispatches │
│  analyzers, builds ProjectLayout            │
└────────────────┬────────────────────────────┘
                 │ uses
┌────────────────▼────────────────────────────┐
│  Infrastructure Layer  (Adapters)           │
│    ├── PythonParser  (rustpython-parser)    │
│    │   implements SourceCodeAnalyzer        │
│    └── MarkdownWriter                       │
│        implements OutputWriter,             │
│                  IndexWriter                │
└────────────────┬────────────────────────────┘
                 │ produces
┌────────────────▼────────────────────────────┐
│  Core Domain   (Entities + Ports)           │
│  entities/  DocumentedConstruct,            │
│             ParsedSourceFile, ProjectLayout │
│             SourceFileAnalysis              │
│  ports/     SourceCodeAnalyzer, ParserError │
│             OutputWriter, OutputWriterError │
│             IndexWriter                     │
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

- **Last task completed:** 07+10 — Delivery layer: `delivery/factories.rs` (wiring) + CLI in `main.rs`
- **Tasks completed before:** 05 — IndexWriter; 04 — MarkdownWriter; 06 — OutputWriter + OutputWriterError; 03 — FileScanner

### Prossimi task pianificati

**Task 11 — Dependency graph (analisi delle dipendenze)**
Obiettivo: ordinare le cartelle nell'INDEX per ordine topologico (domain → infra → application → delivery).
Architettura:
- `domain/entities/dependency_graph.rs` — nuova entità `DependencyGraph`
- `domain/entities/parsed_source_file.rs` + `source_file_analysis.rs` — aggiungere `imports: Vec<String>`
- `domain/ports/index_writer.rs` — aggiornare firma: `write_index(..., graph: &DependencyGraph)`
- `application/analyze_dependencies.rs` — nuovo use case: `ProjectLayout → DependencyGraph`
- `infrastructure/markdown_writer.rs` — usa `DependencyGraph` per sort topologico in `render_index`
- `delivery/factories.rs` + `delivery/run.rs` — wire il nuovo use case
Funzionalità future sbloccate da questo task:
  - Rilevamento cicli (violazioni architetturali)
  - Validazione layer rules (domain non può importare da infra)
  - Analisi impatto: dato un file modificato, trovare tutti i dipendenti
  - Arricchimento INDEX con `depends on` / `imported by`
Problemi di design da tenere a mente:
  - La risoluzione degli import è contestuale: per mappare `from domain.X import Y` → dir `domain/`, il graph builder deve conoscere le dir top-level del ProjectLayout
  - L'analisi è cross-file: `AnalyzeDependencies` riceve l'intero `ProjectLayout`, non un singolo file
  - Il sort topologico appartiene al use case, non all'adapter

**Task 12 — Modalità index-only (CLI flag `--index-only`)**
Genera solo `INDEX.md` senza il mirror dei file. Richiede: nuovo flag in `main.rs`, nuovo use case o branch in `DocumentationGenerator`, factory aggiornata.

**Task 13 — Nuovi language parser**
Aggiungere supporto a un secondo linguaggio (es. Rust o TypeScript). Richiede solo un nuovo adapter in `infrastructure/` registrato in `make_file_scanner()`.

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

# Arcuate

Semantic bridge between source code and its conceptual representation.
Scans a codebase and generates a mirror of Markdown files optimised for LLM consumption: signatures, docstrings, and links to the originals — without exposing full source unless needed.
Produces an `INDEX.md` with a full project tree and dual links (`.md` docs + source file) for every node.

## Usage

```bash
arcuate [--input-dir <path>] [--output-dir <path>]
```

| Flag | Default | Description |
|---|---|---|
| `--input-dir` | current directory | Root of the project to scan |
| `--output-dir` | `<input-dir>/arcuate-docs` | Where to write the generated Markdown files |

**Examples:**

```bash
# Scan current directory, output to ./arcuate-docs
arcuate

# Scan a specific project
arcuate --input-dir ~/projects/myapp

# Specify both input and output
arcuate --input-dir ~/projects/myapp --output-dir ~/docs/myapp
```

## Build

### Development build

```bash
cargo build
```

Produces `target/debug/arcuate`. Unoptimised, includes debug symbols. Use during development.

### Production build

```bash
cargo build --release
```

Produces `target/release/arcuate`. Fully optimised, no debug symbols. The only file you need to distribute or install.

## Install as a global command

To call `arcuate` from any directory, copy the release binary into a directory that is on your `PATH`.

**Option 1 — `/usr/local/bin` (system-wide, requires sudo):**

```bash
sudo cp target/release/arcuate /usr/local/bin/
```

**Option 2 — `~/.local/bin` (user-only, no sudo):**

```bash
mkdir -p ~/.local/bin
cp target/release/arcuate ~/.local/bin/
```

Then make sure `~/.local/bin` is on your `PATH`. Add this line to your `~/.zshrc` (or `~/.bashrc`) if it isn't already:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Reload the shell:

```bash
source ~/.zshrc
```

**Option 3 — via Cargo (recommended):**

```bash
cargo install --path .
```

Cargo builds in release mode and copies the binary to `~/.cargo/bin/`, which is already on your `PATH` if you installed Rust via `rustup`. No extra configuration needed.

Verify the installation:

```bash
which arcuate
arcuate --help
```

# Development

## Commands

```console
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## Layout

```
Cargo.toml
src/main.rs      the tabulate binary
docs/            this documentation (mdBook)
```

## Adding another tool

Each file in `src/bin/` becomes its own binary, named after the file. No
`Cargo.toml` change is needed.

```console
touch src/bin/newtool.rs
cargo run --bin newtool -- --help
cargo install --path . --bin newtool
```

Add `src/lib.rs` when binaries need to share code. To place a binary outside
`src/bin/`, declare it explicitly:

```toml
[[bin]]
name = "newtool"
path = "src/tools/newtool.rs"
```

## Building this documentation

```console
cargo install mdbook --locked
mdbook serve docs
```

Then open <http://localhost:3000>. The built site lands in `docs/book/`, which is
git-ignored. Pushing to `main` deploys it to GitHub Pages.

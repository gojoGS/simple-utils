# Installation

## From source

```console
git clone https://github.com/gojoGS/simple-utils
cd simple-utils
cargo install --path .
```

This places `tabulate` in `~/.cargo/bin`. Make sure that directory is on your
`PATH`.

## Build without installing

```console
cargo build --release
./target/release/tabulate --help
```

## Requirements

- Rust 1.85 or newer (the crate uses edition 2024).
- No runtime dependencies.

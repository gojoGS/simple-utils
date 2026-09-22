# tabulate

Small CLI that reads lines from stdin and aligns their fields into vertical
columns.

```
$ printf 'name age city\nalice 30 NYC\nbob 5 SF\n' | tabulate
name   age  city
alice  30   NYC
bob    5    SF
```

Fields can contain spaces only when you pick an explicit delimiter (`--delimiter ,`),
because the default mode splits on runs of whitespace.

The first non-blank line's indentation is captured and reapplied to every
non-blank line, so indented input stays indented:

```
$ printf '    first\n    second\n  third\n' | tabulate
    first
    second
    third
```

## Usage

```
tabulate [OPTIONS]

Options:
  -d, --delimiter <CHAR>   Split on this exact character instead of runs of whitespace
      --separator <STR>    String placed between output columns [default: "  "]
  -h, --help               Print help
  -V, --version            Print version
```

## Behaviour

- Columns are padded to the width of the widest cell in that column.
- The first non-blank line's indentation is reapplied to every non-blank line;
  blank lines stay blank. Leading whitespace is not counted as a column.
- The last field on each line is never padded, so no trailing whitespace.
- Blank input lines produce blank output lines.
- Ragged rows are fine; short rows simply stop early.
- Width is measured in `char`s, so basic multi-byte text aligns.

## Install

```
cargo install --path .
```

## Add another tool

```sh
# new binary: file name == binary name, auto-detected by cargo
touch src/bin/newtool.rs

# optional: shared code across binaries
touch src/lib.rs
```

```sh
# run / build / test / install one binary
cargo run --bin newtool -- --help
cargo build --bin newtool
cargo test --bin newtool
cargo install --path . --bin newtool

# build/install every binary + lib
cargo build --bins
cargo install --path .
```

```toml
# only needed if the bin lives outside src/bin/
[[bin]]
name = "newtool"
path = "src/tools/newtool.rs"
```

## Develop

```
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

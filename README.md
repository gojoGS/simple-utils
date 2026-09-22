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
- The last field on each line is never padded, so no trailing whitespace.
- Blank input lines produce blank output lines.
- Ragged rows are fine; short rows simply stop early.
- Width is measured in `char`s, so basic multi-byte text aligns.

## Install

```
cargo install --path .
```

## Develop

```
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

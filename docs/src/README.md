# simple-utils

A small collection of focused command-line utilities written in Rust.

Currently ships one tool:

- **`tabulate`** — reads lines from stdin and aligns their fields into vertical
  columns.

## Quick example

```console
$ printf 'name age city\nalice 30 NYC\nbob 5 SF\n' | tabulate
name   age  city
alice  30   NYC
bob    5    SF
```

Indented input keeps its indentation:

```console
$ printf '    first\n    second\n  third\n' | tabulate
    first
    second
    third
```

## Where to go next

- [Installation](installation.md)
- [Usage](usage.md)
- [Behaviour](behaviour.md)
- [Editor integration](editor-integration.md)
- [Development](development.md)

Source: <https://github.com/gojoGS/simple-utils>

# Behaviour

## Columns

- Each field is padded to the width of the widest cell in its column.
- The last field on a line is never padded, so output has no trailing
  whitespace.
- Ragged rows are allowed; a short row simply stops early.
- Width is measured in Unicode `char`s, so basic multi-byte text aligns. Full
  East-Asian display width is not handled.

## Splitting

- By default, lines split on runs of whitespace, and leading/trailing
  whitespace is ignored.
- With `--delimiter <CHAR>`, lines split on that exact character and each field
  is trimmed.
- Consequence: multi-word values such as `San Francisco` become two columns
  unless you pick a delimiter.

## Indentation

The first non-blank line's leading whitespace is captured and reapplied to every
non-blank line. Blank lines stay blank.

```console
$ printf '    first\n    second\n  third\n' | tabulate
    first
    second
    third
```

- The indent is treated as a literal string, so tabs stay tabs.
- Leading whitespace is not counted as a column.
- If the first non-blank line is unindented, the output is unindented.

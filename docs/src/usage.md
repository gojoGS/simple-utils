# Usage

```console
tabulate [OPTIONS]
```

Reads from stdin and writes to stdout, so it is designed to be used as a filter.

## Options

| Option | Description |
|---|---|
| `-d, --delimiter <CHAR>` | Split on this exact character instead of runs of whitespace. |
| `--separator <STR>` | String placed between output columns. Default is two spaces. |
| `-h, --help` | Print help. |
| `-V, --version` | Print version. |

## Examples

Basic alignment:

```console
$ printf 'name age city\nalice 30 NYC\nbob 5 SF\n' | tabulate
name   age  city
alice  30   NYC
bob    5    SF
```

Custom delimiter, useful for CSV:

```console
$ printf 'a,b,cc\nlong,mid,y\n' | tabulate --delimiter ,
a     b    cc
long  mid  y
```

Custom separator:

```console
$ printf 'a bb\nccc d\n' | tabulate --separator ' | '
a   | bb
ccc | d
```

To use it on a selection inside an editor, see
[Editor integration](editor-integration.md).

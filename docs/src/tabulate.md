# `tabulate` 

## Behavior

Formats input lines into tabular format. Token columns are vertically aligned, padded.

```bash
$ printf 'name age city\nalice 30 NYC\nbob 5 SF\n' | tabulate
name   age  city
alice  30   NYC
bob    5    SF
```

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

## Neovim and Vim

Tabulate selection:

```vim
:'<,'>!tabulate
```

Whole file:

```vim
:%!tabulate
```

Mapping for visual mode:

```lua
vim.keymap.set("v", "<leader>t", ":<C-u>'<,'>!tabulate<CR>", { desc = "Tabulate selection" })
```



# Editor integration

`tabulate` is a stdin/stdout filter, so it plugs into any editor that can pipe a
range of lines through an external command.

## Neovim and Vim

1. Select lines in visual mode (`V`, then motions such as `j`, `}`, `G`).
2. Press `:` — the range is filled in automatically.
3. Append `!tabulate` and press Enter.

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

Notes:

- The filter operates on whole lines; the `<` and `>` marks snap to line
  boundaries, so a character-wise selection still replaces full lines.
- `u` undoes the replacement.
- If `tabulate` is not on `PATH`, use an absolute path with forward slashes.

## Other editors

Any editor offering "pipe selection through a command" works the same way.

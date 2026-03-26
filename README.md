# Panel Layout Notation (PLN)

A minimal, human-readable notation for describing split-panel layouts. One line of text can define complex arrangements of panels with sizing, nesting, and named regions -- suitable for terminals, editors, tiling window managers, and web UIs.

## Quick examples

```
(editor | terminal)              # side by side
(editor / terminal)              # stacked
(sidebar=80col | main)           # fixed sidebar, flexible main
(editor=3fr | (terminal / files)) # nested with weighted sizes
```

A standard IDE layout -- file tree, two editors, and a terminal:

```
((files=20% | editor1 | editor2)=3fr / terminal)
```

## How it works

- **Panels** are named regions: bare words like `editor` or quoted strings like `"Left Panel"`
- **Splits** divide space horizontally (`|`) or vertically (`/`), always wrapped in parentheses
- **Sizes** are appended with `=`: ratios (`2fr`), fixed (`80col`, `24row`, `200px`), or percentages (`25%`)
- Panels without a size default to `1fr` (equal share of remaining space)

## Repository contents

- [`standard.md`](standard.md) -- the PLN specification (v1)
- [`CHANGELOG.md`](CHANGELOG.md) -- version history

## Ecosystem

PLN is designed to be implemented across many tools. Planned integrations include:

| Project | Description |
|---------|-------------|
| `pln-parse` | CLI parser and validator (Rust) |
| `pln-kitty` | Kitty terminal plugin |
| `pln-tmux` | tmux plugin |
| `pln-zellij` | Zellij plugin |
| `pln-nvim` | Neovim plugin |
| `pln-css` | PLN to CSS Grid converter |

## License

[MIT](LICENSE)

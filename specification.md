# Panel Layout Notation (PLN)

**Spec version: 0.5.0**

A minimal, human-readable standard for describing split-panel screen layouts, suitable for terminals, editors, and similar environments.

---

## Overview

A layout is composed of named **panels** arranged by **splits**. Every split must be wrapped in parentheses. Each panel or group may carry an optional size annotation; unmarked panels default to a size of `1fr`.

---

## Syntax

### Panels

A panel name is either a bare word or a quoted string. Bare words may not contain whitespace or special characters (`|`, `/`, `(`, `)`, `=`, `"`, `'`). Quoted strings are wrapped in `"` or `'` and may contain any character, including spaces and special characters. Use `\"` or `\'` to escape a quote character inside a string of the same type.

```
editor
terminal
"Left Panel"
'Right Panel'
"it's a panel"
'it\'s a panel'
```

### Splits

`|` splits panels horizontally (left and right):

```
(left | right)
```

`/` splits panels vertically (top and bottom):

```
(top / bottom)
```

All splits must be parenthesized. `left|right` is invalid; `(left|right)` is correct. Parentheses around a single item are permitted but optional — `panel` and `(panel)` are equivalent. Whitespace between tokens is ignored.

### Nesting

Groups may be nested freely:

```
(editor|(terminal/files))
```

### Sizes

Append `=value` to any panel or group to set its size within the enclosing split. Panels without a size annotation default to `1fr`.

```
(left=2fr|right)
(editor=3fr|(terminal=2fr/files))
```

---

## Size Values

A size value may be one of five forms:

| Form | Example | Meaning |
|------|---------|---------|
| Ratio | `1fr`, `2fr` | A relative share of the remaining space after fixed values are allocated |
| Fixed column | `80col` | A fixed number of columns; valid only in `\|` (horizontal) splits |
| Fixed row | `24row` | A fixed number of rows; valid only in `/` (vertical) splits |
| Pixel | `200px` | A fixed number of pixels. Implementations that use discrete cells (e.g. terminals) should round down to the nearest cell boundary |
| Percentage | `25%` | A percentage of the total available space |

Fixed, pixel, and percentage values are allocated first. Ratio units then divide whatever space remains.

---

## Examples

Equal horizontal split:
```
(left|right)
```

Equal vertical split:
```
(top/bottom)
```

Nested splits:
```
(editor|(terminal/files))
```

Unequal split:
```
(left=2fr|right)
```

Fixed sidebar, flexible main area:
```
(sidebar=80col|main)
```

Three-way split with fixed outer panels:
```
(nav=60col|content|panel=40col)
```

Percentage-based split:
```
(left=30%|right)
```

Nested layout with sizes at each level:
```
(editor=3fr|(terminal=2fr/files))
```

Standard IDE layout (sidebar, two editors, terminal below):
```
((files=20% | editor1 | editor2)=3fr / terminal)
```

Single-item group (equivalent to bare `panel`):
```
(panel)
```

Sized single panel:
```
(panel=2fr)
```

Quoted panel names:
```
("Left Panel"=2fr | "Right Panel")
```

---

## Grammar

```
layout   = panel | group
group    = hgroup | vgroup
hgroup   = "(" item ("|" item)* ")"
vgroup   = "(" item ("/" item)* ")"
item     = layout [ "=" value ]
panel    = word | quoted
word     = [^\s|/()="']+
quoted   = "\"" ([^"\\] | "\\\"")* "\""
         | "'"  ([^'\\] | "\\'")* "'"
value    = number unit
number   = [0-9]+ ("." [0-9]+)?
unit     = "fr" | "col" | "row" | "px" | "%"
```

Whitespace between tokens is ignored. A layout is either a bare panel name or a parenthesized group. A group with no operator is a single-item group — equivalent to the item itself, but allowing a size annotation such as `(panel=2fr)`. Each group with operators may only contain one operator type — horizontal (`|`) or vertical (`/`) — never both.

---

## Implementation Notes

- **Default size** — any panel or group without a `=value` annotation has an implicit size of `1fr`.
- **Over/underflow** — if the fixed, pixel, and percentage sizes in a split exceed or fall short of the available space, the behavior is implementation-defined.
- **Unit validation** — `col` is valid only in `|` splits; `row` is valid only in `/` splits. Using the wrong unit for a split direction is an error.
- **Non-character-grid environments** — in environments without a character grid (e.g. web, GUI editors), the mapping of `col` and `row` to pixels is implementation-defined.
- **Panel names** — bare names may not contain whitespace or special characters (`|`, `/`, `(`, `)`, `=`, `"`, `'`). Quoted names may contain any character. Names are case-sensitive.
- **Duplicate names** — whether duplicate panel names within a layout are permitted is implementation-defined.
- **Sizes are scoped** — a `=value` annotation on a group affects only its size within its parent split, not the relative sizes of its own children.

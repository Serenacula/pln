# Screen Layout Notation

A minimal, human-readable standard for describing split-panel screen layouts, suitable for terminals, editors, and similar environments.

---

## Overview

A layout is composed of named **panels** arranged by **splits**. Every split must be wrapped in parentheses. Each panel or group may carry an optional size annotation; unmarked panels default to a size of `1`.

---

## Syntax

### Panels

A panel is any single word. The word is the panel's name.

```
editor
terminal
sidebar
```

### Splits

`|` splits panels horizontally (left and right):

```
(left|right)
```

`/` splits panels vertically (top and bottom):

```
(top/bottom)
```

All splits must be parenthesized. `left|right` is invalid; `(left|right)` is correct.

### Nesting

Groups may be nested freely:

```
(editor|(terminal/files))
```

### Sizes

Append `=value` to any panel or group to set its size within the enclosing split. Panels without a size annotation default to `1`.

```
(left=2|right)
(editor=3|(terminal=2/files))
```

---

## Size Values

A size value may be one of three forms:

| Form | Example | Meaning |
|------|---------|---------|
| Ratio unit | `1`, `2` | A relative share of the remaining space after fixed values are allocated |
| Fixed unit | `80col`, `24row` | A fixed number of units. `col` and `row` are equivalent; the split direction determines which dimension is used |
| Percentage | `25%` | A percentage of the total available space |

Fixed and percentage values are allocated first. Ratio units then divide whatever space remains.

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
(left=2|right)
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
(editor=3|(terminal=2/files))
```

---

## Grammar

```
layout   = panel | group
group    = "(" item (operator item)+ ")"
item     = layout [ "=" value ]
operator = "|" | "/"
panel    = word
value    = number | number unit
unit     = "col" | "row" | "%"
```

A layout is either a bare panel name or a parenthesized group. Mixing `|` and `/` within a single group is invalid.

---

## Implementation Notes

- **Default size** — any panel or group without a `=value` annotation has an implicit size of `1`.
- **All-fixed splits** — if every item in a split has a fixed or percentage size, leftover space is unused and overflow extends past the available area. Implementations are not required to handle this gracefully.
- **Panel names** — any word that does not contain `|`, `/`, `(`, `)`, `=`, or `:`. Case-sensitive.
- **Sizes are scoped** — a `=value` annotation on a group affects only its size within its parent split, not the relative sizes of its own children.

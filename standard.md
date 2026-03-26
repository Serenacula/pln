# Screen Layout Notation

A minimal, human-readable standard for describing split-panel screen layouts, suitable for terminals, editors, and similar environments.

---

## Overview

A layout is composed of named **panels** arranged by **splits**. Every split must be wrapped in parentheses. This makes layouts unambiguous and straightforward to parse.

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

### Ratios

Append `=ratio` to a group to control how space is divided among its panels. The ratio contains one value per panel, separated by `:`.

```
(left|right)=2:1
(editor|(terminal/files)=2:1)=3:1
```

If no ratio is specified, equal division is assumed — equivalent to `1:1:...:1`.

---

## Size Values

A ratio value may be one of three forms:

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
(left|right)=2:1
```

Fixed sidebar, flexible main area:
```
(sidebar|main)=80col:1
```

Three-way split with fixed outer panels:
```
(nav|content|panel)=60col:1:40col
```

Percentage-based split:
```
(left|right)=30%:1
```

Nested layout with ratios at each level:
```
(editor|(terminal/files)=2:1)=3:1
```

---

## Grammar

```
layout   = panel | group
group    = "(" layout (operator layout)+ ")" [ "=" ratio ]
operator = "|" | "/"
panel    = word
ratio    = value (":" value)*
value    = number | number unit
unit     = "col" | "row" | "%"
```

A layout is either a bare panel name or a parenthesized group. Mixing `|` and `/` within a single group is invalid.

---

## Implementation Notes

- **All-fixed splits** — if every value in a ratio is fixed or percentage, leftover space is unused and overflow extends past the available area. Implementations are not required to handle this gracefully.
- **Panel names** — any word that does not contain `|`, `/`, `(`, `)`, `=`, or `:`. Case-sensitive.
- **Ratios are scoped** — a `=ratio` annotation applies only to the split directly inside its group, not to nested splits.
- **Default ratio** — an unannotated group divides space equally across all its direct children.

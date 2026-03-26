# Changelog

All notable changes to the PLN specification will be documented in this file.

This project follows [Semantic Versioning](https://semver.org/) and the format is based on [Keep a Changelog](https://keepachangelog.com/).

## [0.5.0] - 2026-03-26

### Added
- Single-item groups: `(panel)` is valid and equivalent to `panel`, allowing `(panel=2fr)` for sizing a standalone panel

## [0.4.0] - 2026-03-26

### Added
- Formal grammar definition
- Quoted panel names using `"` or `'` with escape support
- Pixel (`px`) size unit for non-character-grid environments
- Fractional ratio unit (`fr`) replacing bare numbers for sizes
- Split-direction-specific unit validation (`col` for `|`, `row` for `/`)
- Implementation notes for over/underflow, duplicate names, and non-character-grid environments

### Changed
- Default size is now `1fr` (previously `1`)
- Size values now require an explicit unit (`2fr` instead of `2`)

## [0.2.0] - 2026-03-26

### Changed
- Size annotations moved from groups to individual panels

## [0.1.1] - 2026-03-26

### Changed
- All splits must be wrapped in parentheses, removing parsing ambiguity

## [0.1.0] - 2026-03-26

### Added
- Initial specification: panels, horizontal/vertical splits, nesting, ratio/fixed/percentage sizes

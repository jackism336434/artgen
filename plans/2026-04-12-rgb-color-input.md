# RGB Color Input

## Goal

Extend `artgen` color parsing so `--color`, `--from`, and `--to` accept `rgb(r,g,b)` and `r,g,b` in addition to named colors and `#RRGGBB`.

## Key Changes

- Reuse the existing `CliColor` parser and add two RGB literal formats
- Accept `rgb(255,102,0)` and `255,102,0` with channel values in `0..=255`
- Keep all current CLI flags, animations, gradients, and default behaviors unchanged
- Update README and project status docs to describe the expanded color syntax

## Test Plan

- Parse `rgb(...)` and comma-separated RGB for `--color`
- Parse RGB literals for `--from` / `--to`
- Reject missing channels, extra channels, non-numeric values, and out-of-range values
- Keep named colors and `#RRGGBB` behavior unchanged

## Assumptions

- Spaces around RGB numbers and commas are accepted
- `rgb(...)` is case-insensitive
- Percent-based RGB values are not supported

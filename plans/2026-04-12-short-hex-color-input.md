# Short Hex Color Input

## Goal

Extend `artgen` color parsing so `--color`, `--from`, and `--to` accept `#RGB` in addition to named colors, `#RRGGBB`, `rgb(r,g,b)`, and `r,g,b`.

## Key Changes

- Reuse the existing `CliColor` parser and add short hex expansion
- Accept `#RGB` by duplicating each nibble into the full RGB byte
- Keep all current CLI flags, animations, gradients, and default behaviors unchanged
- Update README and project status docs to describe the expanded color syntax

## Test Plan

- Parse lowercase and uppercase `#RGB`
- Parse `#RGB` for `--color`, `--from`, and `--to`
- Reject invalid short hex strings such as `#ggg` and malformed lengths
- Keep named colors, `#RRGGBB`, `rgb(...)`, and `r,g,b` behavior unchanged

## Assumptions

- `#RGB` follows standard expansion, for example `#f60 -> #ff6600`
- Four-digit or eight-digit hex formats are not supported

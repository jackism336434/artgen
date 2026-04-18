# Hex Color Input

## Goal

Allow `artgen` to accept `#RRGGBB` color input for `--color`, `--from`, and `--to` while keeping existing named colors working.

## Key Changes

- Replace CLI color parameters based on `ValueEnum` with a parsed color input type
- Support both named colors and `#RRGGBB` in the same arguments
- Reuse the parsed RGB values for solid output and custom gradients
- Keep preset gradients, animation behavior, and existing flag relationships unchanged
- Update README and project status docs to reflect the new CLI behavior

## Test Plan

- Parse named colors and hex colors for `--color`
- Parse hex endpoints for `--from` / `--to`
- Reject invalid hex strings and missing `#`
- Keep existing mutual exclusion and dependency validation working
- Verify custom gradients preserve exact start and end RGB values

## Assumptions

- Only `#RRGGBB` is supported in this round
- Hex parsing is case-insensitive
- Named colors remain supported for backward compatibility

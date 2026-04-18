# Alpha Color Input

## Goal

Extend `artgen` color parsing so `--color`, `--from`, and `--to` accept `#RGBA` and `#RRGGBBAA`, with alpha composited against a black background.

## Key Changes

- Reuse the existing `CliColor` parser and add alpha-aware hex formats
- Accept `#RGBA` and `#RRGGBBAA` by blending the parsed RGB value against black using the alpha channel
- Keep all current CLI flags, animations, gradients, and default behaviors unchanged
- Update README and project status docs to describe the expanded color syntax and black-background alpha rule

## Test Plan

- Parse lowercase and uppercase `#RGBA` and `#RRGGBBAA`
- Parse alpha hex formats for `--color`, `--from`, and `--to`
- Verify full alpha preserves the original RGB and zero alpha becomes black
- Reject malformed alpha hex strings

## Assumptions

- Alpha is composited against black, since terminal foreground colors do not support transparency
- Four-digit and eight-digit alpha hex formats are the only alpha-enabled formats in this round

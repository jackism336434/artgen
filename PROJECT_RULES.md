# Project Rules

## Purpose
This file stores the long-lived collaboration rules for `artgen`.

Do not use chat history as the source of truth for project workflow. Use the repository files defined below.

## Required Startup Routine
At the start of every new session, read these files in order:

1. `PROJECT_RULES.md`
2. `CURRENT_STATUS.md`
3. Relevant recent files under `plans/`

This is the default way to restore project context and task progress.

## Planning Rule
Before implementing any new feature, bug fix, or meaningful refactor:

- create a new plan file under `plans/`
- write the intended change before implementation starts

The plan file should describe:

- goal
- key changes
- test plan
- assumptions

## Plan File Convention
Store one task per file under `plans/`.

Recommended filename format:

- `YYYY-MM-DD-topic.md`

Examples:

- `2026-04-04-cli-text-input.md`
- `2026-04-04-rainbow-gradient.md`

## Completion Routine
After finishing a task:

- update `CURRENT_STATUS.md`
- record what was completed
- record the next recommended step
- if useful, mark the related plan file with a simple status such as `planned`, `in_progress`, or `done`

## Scope Separation
- `PROJECT_RULES.md` stores stable collaboration rules
- `CURRENT_STATUS.md` stores the latest project snapshot
- `plans/` stores individual task plans and their history
- `src/plan.md` stores the broader project direction, not day-to-day task tracking

## Default Assumption
If a new session starts and the user asks to continue work, first recover context from the files above instead of relying on memory.

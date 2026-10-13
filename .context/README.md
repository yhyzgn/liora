# Liora CTX Context

## Canonical reading order

1. `AGENTS.md`
2. `.context/README.md`
3. Current plan: `.context/plans/004_gpui_native_replacement_20261013.md`
4. Current task: `.context/tasks/004_gpui_native_replacement_20261013.md`
5. `.context/system/overview.md`
6. `.context/system/conventions.md`
7. `.context/system/risks.md`

## Directory responsibilities

- `system/`: stable verified facts, conventions, and risks.
- `plans/`: bounded phase plans and migrated phase history.
- `tasks/`: bounded execution tasks and migrated session history.
- `archive/`: byte-preserving legacy source archive after migration finalization; not a primary reading source.

## Lifecycle

- Plan status: `草稿 -> 进行中 -> 暂停 -> 已完成 | 已取代`.
- Task status: `待开始 -> 进行中 -> 已完成 | 阻塞 | 已取代`.
- Update stable facts in `system/`; update plan/task files when scope, acceptance, validation, or status changes.

## Current pointers

The authoritative current plan/task pointers live in `AGENTS.md` and currently point to `.context/plans/004_gpui_native_replacement_20261013.md` and `.context/tasks/004_gpui_native_replacement_20261013.md`.

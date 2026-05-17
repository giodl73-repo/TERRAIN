# Pulse 03 - Role review and docs

## Goal

Document manager exception register semantics and role-review outcomes.

## Scope

- Update README and graph/review docs.
- Record `.roles` findings.
- Close Phase 11 after validation.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

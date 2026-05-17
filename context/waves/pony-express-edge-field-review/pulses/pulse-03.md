# Pulse 03 - Role review and docs

## Goal

Document the manager-language boundary for edge field review.

## Scope

- Update README and graph contract docs with the new commands.
- Record `.roles` review findings for manager review language.
- Close Phase 10 when validation passes.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
git diff --check
```

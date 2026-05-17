# Pulse 03: Role Review and Docs

Status: done

## Goal

Document edge audit semantics and review boundaries.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`

## Outcome

- Documented territory edge audit semantics in `docs/site-graph-contract.md`.
- Clarified that cut edges are review signals, not automatic failures.

# Pulse 03: Cached Geography Intake

Status: done

## Goal

Evaluate CROP, PEBBLE, and FLETCH as reusable fixture and cached-data handoff
points without adding premature dependencies.

## Outcome

- Added `docs\integration-fixtures.md`.
- Added a machine-readable `terrain.integration-fixtures.v1` manifest.
- Added `terrain-cli integration-fixtures` for downstream tools.
- Identified CROP, PEBBLE, and FLETCH as candidate handoff sources while keeping
  territory policy local.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- integration-fixtures`
- `git diff --check`

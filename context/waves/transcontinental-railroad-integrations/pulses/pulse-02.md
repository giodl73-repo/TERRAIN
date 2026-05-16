# Pulse 02: Dashboard Schema Contract

Status: done

## Goal

Stabilize CSV, SVG, GeoJSON, and packet field names for BI consumers.

## Outcome

- Added `docs\dashboard-schema.md`.
- Added a machine-readable `terrain.dashboard.v1` schema contract in
  `terrain-core`.
- Added a `terrain-cli schema` command for downstream dashboards and tests.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- schema`
- `git diff --check`

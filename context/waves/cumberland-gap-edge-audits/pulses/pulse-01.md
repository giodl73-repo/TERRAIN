# Pulse 01: Territory Edge Audit CLI

Status: done

## Goal

Report cut edges, unknown edge sites, and disconnected territory components from
territory CSV plus edge evidence CSV.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-csv > sample-territories.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- territory-edge-audit-csv sample-territories.csv sample-site-edges.csv`
- `git diff --check`

## Outcome

- Added `territory_edge_audit`.
- Added `territory-edge-audit-csv`.
- Reports `cut-edge`, `unknown-edge-site`, and `disconnected-territory`.

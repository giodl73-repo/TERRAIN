# Pulse 02: Graph Diagnostics CLI

Status: done

## Goal

Expose graph-construction diagnostics from site CSVs before any graph-backed
partitioning is used for planning decisions.

## Scope

- Added `graph-diagnostics-csv PATH [LONG_EDGE_THRESHOLD]`.
- Reports node count, edge count, component count, diagnostic count, isolated
  sites, duplicate coordinates, disconnected components, and long-edge warnings.
- Keeps output as a summary line plus plain CSV diagnostic rows for joins.
- Added core tests for deterministic graph diagnostic report ordering.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- graph-diagnostics-csv sample-sites.csv`
- `git diff --check`


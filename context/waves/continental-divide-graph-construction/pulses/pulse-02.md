# Pulse 02: Graph Diagnostics CLI

Status: planned

## Goal

Expose graph-construction diagnostics from site CSVs before any graph-backed
partitioning is used for planning decisions.

## Scope

- Add a CLI command for graph diagnostics over `sample-sites.csv` and user CSVs.
- Report component count, isolated sites, duplicate coordinates, and long-edge
  warnings.
- Keep output plain CSV/text so it can join existing packet and dashboard flows.
- Add fixtures that prove deterministic diagnostic ordering.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- graph-diagnostics-csv sample-sites.csv`
- `git diff --check`


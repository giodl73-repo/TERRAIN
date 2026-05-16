# Pulse 04: Field Review Export

Status: done

## Goal

Emit a plain-language scenario review that field managers can read without
interpreting raw balance or movement tables.

## Outcome

- Added a field review CLI command for baseline/proposed territory CSVs.
- Summarizes recommendation, balance status, site movement, and compactness
  exceptions.
- Keeps the field review connected to the same stable site and territory IDs
  used by CSV, SVG, and GeoJSON outputs.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- field-review-csv fixtures\sample-territories.csv fixtures\sample-territories-proposed.csv`
- `git diff --check`

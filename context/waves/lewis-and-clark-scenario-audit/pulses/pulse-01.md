# Pulse 01: Scenario Comparison

Status: done

## Goal

Compare a baseline territory plan against a proposed plan with explainable
audit and territory-level deltas.

## Outcome

- Added core scenario comparison types.
- Added baseline/proposed audit comparison.
- Added per-territory site count, demand, and revenue deltas.
- Added a proposed-plan CSV fixture and CLI comparison command.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-proposed-csv`
- `cargo run -p terrain-cli -- compare-csv fixtures\sample-territories.csv fixtures\sample-territories-proposed.csv`
- `git diff --check`

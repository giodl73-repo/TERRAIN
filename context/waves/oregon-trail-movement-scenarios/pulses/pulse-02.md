# Pulse 02: Scenario Count Sweep

Status: done

## Goal

Compare deterministic partitions across a range of target territory counts from
the same unassigned site input.

## Outcome

- Added core partition sweep results with target count, actual count, and audit.
- Added a CLI count-sweep command for unassigned site CSV rows.
- Kept the sweep deterministic so scenario rows can be compared in dashboards.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sweep-csv fixtures\sample-sites.csv 2 3`
- `git diff --check`

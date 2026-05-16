# Pulse 03: First Partition

Status: done

## Goal

Produce a deterministic first-cut territory assignment from unassigned site rows,
then reuse the audit and visual contracts from the earlier pulses.

## Outcome

- Added parsing for unassigned site CSV rows with `site_id`, `demand`,
  `revenue`, `latitude`, and `longitude`.
- Added a deterministic demand-balancing partitioner that creates stable
  territory IDs and labels.
- Added CLI commands for partition audit and SVG export.
- Added a fixture for unassigned site rows.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv`
- `cargo run -p terrain-cli -- partition-csv fixtures\sample-sites.csv 2`
- `cargo run -p terrain-cli -- partition-svg-csv fixtures\sample-sites.csv 2`
- `git diff --check`

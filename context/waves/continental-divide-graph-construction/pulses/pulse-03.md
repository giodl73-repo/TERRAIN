# Pulse 03: Graph-backed Partition Baseline

Status: done

## Goal

Add a graph-backed partition path that can be compared against the existing
deterministic greedy partition.

## Scope

- Keep `partition_sites` as the stable baseline.
- Add a graph construction step for site CSVs.
- Produce a partition comparison report with balance, compactness, movement, and
  component diagnostics.
- Do not adopt METIS-CORE until graph input tests are stable.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- partition-csv sample-sites.csv 2`
- `cargo run -p terrain-cli -- graph-partition-csv sample-sites.csv 2`
- `git diff --check`

## Outcome

- Added an internal graph-seeded partition path while keeping `partition_sites`
  as the stable greedy baseline.
- Added a graph partition comparison report with balance deltas, movement rows,
  compactness exceptions, and graph diagnostics.
- Exposed the report through `graph-partition-csv`.


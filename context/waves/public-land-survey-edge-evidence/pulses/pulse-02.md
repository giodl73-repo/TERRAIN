# Pulse 02: Evidence-backed METIS Handoff

Status: done

## Goal

Build METIS-CORE handoff and partitioning from explicit edge evidence instead
of coordinate-complete graphs.

## Scope

- Add an edge-backed CSR handoff using parsed site edge evidence.
- Add an edge-backed METIS partition command.
- Preserve coordinate-derived METIS handoff and partitioning as the default.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- metis-handoff-with-edges-csv sample-sites.csv sample-site-edges.csv`
- `cargo run -p terrain-cli -- metis-partition-with-edges-csv sample-sites.csv sample-site-edges.csv 2`
- `git diff --check`

## Outcome

- Added `metis_core_handoff_with_edges`.
- Added `partition_sites_with_metis_core_edges`.
- Added `metis-handoff-with-edges-csv` and `metis-partition-with-edges-csv`.

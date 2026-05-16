# Pulse 04: Visual Export Contract

Status: done

## Goal

Make dashboard-ready visual exports a reusable product contract across assigned
territory CSVs and deterministic partitions.

## Outcome

- Added GeoJSON FeatureCollection rendering from the core territory model.
- Added territory centroid features with shared territory IDs, labels, counts,
  demand, revenue, assignee, and compactness properties.
- Added site point features with shared territory IDs, site IDs, demand,
  revenue, and assignee properties.
- Added CLI commands for sample, CSV, and partition GeoJSON exports.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-geojson`
- `cargo run -p terrain-cli -- geojson-csv fixtures\sample-territories.csv`
- `cargo run -p terrain-cli -- partition-geojson-csv fixtures\sample-sites.csv 2`
- `git diff --check`

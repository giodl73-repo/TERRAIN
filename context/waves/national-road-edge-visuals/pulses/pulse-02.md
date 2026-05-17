# Pulse 02: Edge Evidence GeoJSON

Status: done

## Goal

Emit edge evidence as GeoJSON LineString features plus site points.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- edge-evidence-geojson-csv sample-sites.csv sample-site-edges.csv > edge-evidence.geojson`
- `git diff --check`

## Outcome

- Added `render_site_graph_geojson`.
- Added `edge-evidence-geojson-csv`.
- GeoJSON includes LineString edge features and site point features.

# Pulse 01: Site Graph Contract

Status: done

## Goal

Define the first explicit graph contract for TERRAIN site data.

## Scope

- Added graph node, edge, graph, and diagnostic structs in `terrain-core`.
- Preserved stable site IDs as graph node IDs.
- Defined first edge weights as latitude-adjusted coordinate distance in degrees
  with `latitude-adjusted-coordinate-distance-degrees` evidence.
- Added graph diagnostics for empty inputs, duplicate IDs, duplicate
  coordinates, and disconnected components.
- Documented the `terrain.site-graph.v1` boundary in
  `docs/site-graph-contract.md`.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv`
- `git diff --check`


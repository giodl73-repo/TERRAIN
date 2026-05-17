# Pulse 01: Site Graph Contract

Status: planned

## Goal

Define the first explicit graph contract for TERRAIN site data.

## Scope

- Add graph node and edge structs in `terrain-core`.
- Preserve stable site IDs as graph node IDs.
- Define edge weights in product-neutral terms such as coordinate distance or
  supplied adjacency cost.
- Add graph diagnostics for empty inputs, duplicate IDs, duplicate coordinates,
  and disconnected components.
- Document which fields are TERRAIN policy versus reusable graph shape.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv`
- `git diff --check`


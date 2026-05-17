# Pulse 02 - Edge field review packet

## Goal

Package edge field-review language with the rows and visuals needed for
operations review.

## Scope

- Add an edge field-review packet command.
- Include field-review text, field action CSV, raw audit CSV, SVG, and GeoJSON.
- Preserve stable site and territory IDs across every artifact.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p terrain-cli -- territory-edge-field-packet-csv sample-territories.csv sample-site-edges.csv target\territory-edge-field-packet-smoke
```

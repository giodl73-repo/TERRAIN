# Pulse 02: Edge Audit Packet Outputs

Status: done

## Goal

Write edge audit rows and visual artifacts into review packets.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-csv > sample-territories.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- territory-edge-packet-csv sample-territories.csv sample-site-edges.csv target\territory-edge-packet-smoke`
- `git diff --check`

## Outcome

- Added `territory-edge-packet-csv`.
- Packet includes territory edge audit rows, source edge evidence, SVG, and
  GeoJSON review artifacts.

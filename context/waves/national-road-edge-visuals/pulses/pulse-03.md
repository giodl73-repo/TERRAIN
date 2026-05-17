# Pulse 03: Edge Visual Packet Outputs

Status: done

## Goal

Include edge evidence SVG and GeoJSON in review packets.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- edge-evidence-packet-csv sample-sites.csv sample-site-edges.csv target\terrain-edge-visual-packet-smoke`
- `git diff --check`

## Outcome

- `edge-evidence-packet-csv` now writes `edge-evidence.svg` and
  `edge-evidence.geojson`.
- Packet output keeps edge evidence rows, diagnostics, and METIS CSR handoff
  tables alongside visual review artifacts.

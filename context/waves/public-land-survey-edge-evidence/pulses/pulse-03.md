# Pulse 03: Edge Evidence Packet Outputs

Status: done

## Goal

Include graph evidence and diagnostics in review packet outputs.

## Scope

- Write supplied edge evidence into a packet.
- Write edge-backed graph diagnostics into a packet.
- Write METIS CSR vertex and adjacency handoff tables into a packet.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- edge-evidence-packet-csv sample-sites.csv sample-site-edges.csv target\terrain-edge-packet-smoke`
- `git diff --check`

## Outcome

- Added `edge-evidence-packet-csv`.
- Packet files include `edge-evidence.csv`, `edge-diagnostics.csv`,
  `metis-handoff-vertices.csv`, and `metis-handoff-adjacency.csv`.

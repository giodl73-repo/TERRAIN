# Pulse 04: Scenario Packet

Status: done

## Goal

Export a reviewable scenario packet that operations and dashboard users can
inspect without rerunning individual commands.

## Outcome

- Added a CLI packet command for baseline/proposed CSV scenarios.
- Writes scenario summary metrics, per-territory deltas, proposed SVG, and
  proposed GeoJSON into an output directory.
- Keeps packet files simple and dashboard-friendly.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- packet-csv fixtures\sample-territories.csv fixtures\sample-territories-proposed.csv target\terrain-packet-smoke`
- `Test-Path target\terrain-packet-smoke\scenario-summary.csv`
- `Test-Path target\terrain-packet-smoke\territory-deltas.csv`
- `Test-Path target\terrain-packet-smoke\proposed.svg`
- `Test-Path target\terrain-packet-smoke\proposed.geojson`
- `git diff --check`

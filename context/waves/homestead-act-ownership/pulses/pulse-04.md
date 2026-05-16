# Pulse 04: Fairness Review Packet

Status: done

## Goal

Package people, capacity, ownership visual bindings, and capacity exceptions
into a review folder for field managers and dashboard builders.

## Outcome

- Added a fairness packet CLI command for territory and capacity CSVs.
- Writes capacity roster, capacity overloads, ownership SVG, and ownership
  GeoJSON into an output directory.
- Keeps ownership/fairness outputs tied to stable territory, site, and assignee
  identifiers.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- fairness-packet-csv fixtures\sample-territories.csv fixtures\sample-capacity.csv target\terrain-fairness-packet-smoke`
- `Test-Path target\terrain-fairness-packet-smoke\capacity-roster.csv`
- `Test-Path target\terrain-fairness-packet-smoke\capacity-overloads.csv`
- `Test-Path target\terrain-fairness-packet-smoke\ownership.svg`
- `Test-Path target\terrain-fairness-packet-smoke\ownership.geojson`
- `git diff --check`

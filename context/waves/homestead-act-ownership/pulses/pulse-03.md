# Pulse 03: Ownership Visual Bindings

Status: done

## Goal

Carry owner and capacity context into visual exports so dashboards can join
territory marks to fairness and workload fields.

## Outcome

- Added capacity-aware SVG export with `data-capacity`, `data-overload`, and
  `data-owner-count` bindings on territory marks.
- Added capacity-aware GeoJSON export with capacity, overload, owner count, and
  assignee properties.
- Added CLI commands for ownership SVG and GeoJSON exports.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- ownership-svg-csv fixtures\sample-territories.csv fixtures\sample-capacity.csv`
- `cargo run -p terrain-cli -- ownership-geojson-csv fixtures\sample-territories.csv fixtures\sample-capacity.csv`
- `git diff --check`

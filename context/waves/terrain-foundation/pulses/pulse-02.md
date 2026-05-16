# Pulse 02: CSV intake

## Goal

Parse customer/job rows with workload, revenue, coordinates, territory, and
assigned people/team fields into the core audit model.

## Changes

- Add CSV intake for `territory_id`, `territory_label`, `assignees`, `site_id`,
  `demand`, `revenue`, `latitude`, and `longitude`.
- Preserve territory order from the CSV.
- Group sites by territory and dedupe assignees.
- Add CLI commands:
  - `sample-csv`
  - `audit-csv PATH`
  - `svg-csv PATH`
- Keep output IDs shared across CSV audit rows and SVG `data-*` bindings.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-csv`
- `cargo run -p terrain-cli -- audit-csv fixtures/sample-territories.csv`
- `cargo run -p terrain-cli -- svg-csv fixtures/sample-territories.csv`
- `git diff --check`

## Status

Done.

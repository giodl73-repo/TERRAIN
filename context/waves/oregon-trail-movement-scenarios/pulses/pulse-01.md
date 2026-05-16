# Pulse 01: Movement Manifest

Status: done

## Goal

Make site movement explicit and traceable when a proposed scenario changes
territory ownership.

## Outcome

- Added core site movement records keyed by stable site ID.
- Added before/after territory IDs and movement kind: unchanged, moved, added,
  or removed.
- Added a CLI movement manifest command for baseline/proposed territory CSVs.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- movement-csv fixtures\sample-territories.csv fixtures\sample-territories-proposed.csv`
- `git diff --check`

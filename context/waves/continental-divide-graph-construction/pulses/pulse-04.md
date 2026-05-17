# Pulse 04: METIS-CORE Adoption Adapter

Status: planned

## Goal

Add a narrow METIS-CORE adoption boundary once TERRAIN has a stable graph input
contract.

## Scope

- Decide whether the first integration is a runtime crate dependency, fixture
  handoff, or benchmark-only comparison.
- Keep customer territory scoring, capacity, fairness, and field-review language
  in TERRAIN.
- Add validation that graph-backed outputs still feed existing dashboard and
  packet commands.
- Update TRACKER dependency usage if METIS-CORE moves from planned to active.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- graph-partition-csv sample-sites.csv 2`
- `cargo run -p terrain-cli -- packet-csv sample-territories.csv sample-territories-proposed.csv target\terrain-graph-packet-smoke`
- `git diff --check`


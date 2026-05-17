# Pulse 04: METIS-CORE Adoption Adapter

Status: done

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
- `cargo run -p terrain-cli -- metis-handoff-csv sample-sites.csv`
- `cargo run -p terrain-cli -- metis-partition-csv sample-sites.csv 2`
- `cargo run -p terrain-cli -- packet-csv sample-territories.csv sample-territories-proposed.csv target\terrain-graph-packet-smoke`
- `git diff --check`

## Decision

The first METIS-CORE integration is a GitHub-backed runtime crate dependency
plus a fixture/benchmark handoff. Portfolio dependencies should flow through
GitHub repositories rather than crates.io publishing. TERRAIN still does not
move customer territory scoring, capacity, fairness, field-review language, or
dashboard policy into a shared graph engine.

## Outcome

- Added `metis-core = { git = "https://github.com/giodl73-repo/METIS-CORE.git" }`
  as TERRAIN's METIS-CORE dependency.
- Added a deterministic CSR handoff contract for METIS-CORE benchmarks.
- Added `metis-handoff-csv` to emit vertex, adjacency, vertex-weight, and
  edge-weight rows from TERRAIN site CSV inputs.
- Added `metis-partition-csv` to run METIS-CORE partitioning and audit the
  result through TERRAIN balance reporting.
- Kept `partition_sites` and `graph-partition-csv` as TERRAIN-owned product
  baselines while making METIS-CORE adoption measurable.


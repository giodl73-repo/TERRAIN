# Pulse 02: Capacity Flags

Status: done

## Goal

Flag territories where assigned people or teams do not have enough capacity for
the territory workload.

## Outcome

- Added capacity exceptions comparing territory demand to summed assignee
  capacity.
- Added CLI reporting for overloaded territories.
- Preserved assignee names so overloads can be traced back to ownership rows.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- capacity-audit-csv fixtures\sample-territories.csv fixtures\sample-capacity.csv`
- `git diff --check`

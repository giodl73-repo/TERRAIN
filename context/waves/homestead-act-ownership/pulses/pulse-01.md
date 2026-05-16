# Pulse 01: Assignee Capacity Intake

Status: done

## Goal

Parse capacity and home-base fields for assigned people or teams so future
fairness checks can compare workload against operational limits.

## Outcome

- Added assignee capacity records with assignee, team, capacity, home latitude,
  home longitude, and skills.
- Added CSV parsing and a built-in capacity fixture.
- Added CLI commands to emit and summarize capacity CSV rows.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-capacity-csv`
- `cargo run -p terrain-cli -- capacity-csv fixtures\sample-capacity.csv`
- `git diff --check`

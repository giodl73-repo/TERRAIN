# Pulse 02: Intake Diagnostics

Status: done

## Goal

Report CSV intake problems without losing row context, so messy real customer or
job exports can be cleaned before audit, partition, or scenario comparison.

## Outcome

- Added structured CSV diagnostics with severity, line, field, and message.
- Added checks for missing required values, duplicate site IDs, invalid numeric
  fields, and latitude/longitude ranges.
- Added a CLI diagnostics command and messy fixture.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- diagnose-csv fixtures\messy-territories.csv`
- `git diff --check`

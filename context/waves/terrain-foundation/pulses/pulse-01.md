# Pulse 01: Workspace foundation

## Goal

Create the repository foundation and first tested contract.

## Changes

- Add Rust workspace skeleton.
- Add README, product plan, CLAUDE context, and license.
- Add wave and pulse scaffolding.
- Add TERRAIN-specific skills.
- Add first balance-audit library contract and sample CLI command.
- Add sample data-bound SVG split output for dashboard embedding.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-audit`
- `cargo run -p terrain-cli -- sample-svg`
- `git diff --check`

## Status

Done.

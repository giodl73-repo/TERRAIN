# Pulse 01 - Register model

## Goal

Build a combined exception register across TERRAIN review surfaces.

## Scope

- Add a manager exception register model.
- Include balance, movement, compactness, capacity, and edge findings.
- Preserve stable IDs and action language for each row.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p terrain-cli -- manager-exception-register-csv sample-territories.csv sample-territories-proposed.csv sample-capacity.csv sample-site-edges.csv
```

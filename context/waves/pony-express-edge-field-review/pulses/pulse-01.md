# Pulse 01 - Edge field review recommendations

## Goal

Translate territory edge audit findings into plain-language recommendations and
actions for field managers.

## Scope

- Add a field-review model over territory edge audit diagnostics.
- Recommend ready, manager-review, or input-fix status.
- Explain cut edges, disconnected territories, and unknown edge sites without
  graph vocabulary.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p terrain-cli -- territory-edge-field-review-csv sample-territories.csv sample-site-edges.csv
```

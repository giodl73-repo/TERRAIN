# Pulse 02 - Register packet

## Goal

Package the manager exception register with summary text and visuals.

## Scope

- Add a packet command for the register.
- Include register CSV, summary text, territory SVG/GeoJSON, and edge SVG/GeoJSON.
- Keep visual artifacts as review context rather than routing output.

## Validation

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p terrain-cli -- manager-exception-packet-csv sample-territories.csv sample-territories-proposed.csv sample-capacity.csv sample-site-edges.csv target\manager-exception-packet-smoke
```

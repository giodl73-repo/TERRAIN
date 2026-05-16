# Pulse 03: Exception Packet

Status: done

## Goal

Combine the movement, balance, compactness, and intake-diagnostics outputs into
one scenario review packet.

## Outcome

- Expanded `packet-csv` from a visual/scenario packet into an exception packet.
- Added movement manifest, baseline diagnostics, proposed diagnostics, and
  compactness exception CSV files to the packet.
- Kept SVG and GeoJSON exports in the same packet for dashboard handoff.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- packet-csv fixtures\sample-territories.csv fixtures\sample-territories-proposed.csv target\terrain-packet-smoke`
- `Test-Path target\terrain-packet-smoke\movement-manifest.csv`
- `Test-Path target\terrain-packet-smoke\compactness-exceptions.csv`
- `git diff --check`

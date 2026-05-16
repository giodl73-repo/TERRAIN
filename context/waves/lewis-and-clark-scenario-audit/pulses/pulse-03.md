# Pulse 03: Compactness Checks

Status: done

## Goal

Surface simple compactness and travel-proxy exceptions so a balanced scenario
can still be flagged for field review.

## Outcome

- Added compactness exceptions based on each territory's max radius from its
  centroid.
- Added CLI reporting for territories whose max radius exceeds a caller-provided
  threshold.
- Kept the check local to TERRAIN while RLINE/shared graph adoption remains a
  later phase.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- compactness-csv fixtures\sample-territories.csv 0.06`
- `git diff --check`

# Pulse 01: Site Edge Evidence CSV

Status: done

## Goal

Add explicit site-to-site edge evidence intake for graph diagnostics.

## Scope

- Add a site-edge CSV contract with `from_site_id`, `to_site_id`, `weight`, and
  `evidence`.
- Build a site graph from explicit edge rows while keeping coordinate-derived
  graph construction as the default.
- Diagnose unknown site references, duplicate edge rows, disconnected
  components, isolated sites, and long edges.
- Add CLI commands for sample edge evidence and edge-backed diagnostics.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- graph-diagnostics-with-edges-csv sample-sites.csv sample-site-edges.csv`
- `git diff --check`

## Outcome

- Added `sample-site-edges-csv` for explicit edge evidence fixtures.
- Added `graph-diagnostics-with-edges-csv` for edge-backed graph diagnostics.
- Added parsing and diagnostics for unknown site references, self edges,
  duplicate edges, disconnected components, isolated sites, and long edges.

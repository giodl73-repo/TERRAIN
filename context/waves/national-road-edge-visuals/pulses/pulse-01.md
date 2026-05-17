# Pulse 01: Edge Evidence SVG

Status: done

## Goal

Render explicit edge evidence as dashboard-bound SVG lines and site marks.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv`
- `cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv`
- `cargo run -p terrain-cli -- edge-evidence-svg-csv sample-sites.csv sample-site-edges.csv > edge-evidence.svg`
- `git diff --check`

## Outcome

- Added `render_site_graph_svg`.
- Added `edge-evidence-svg-csv`.
- SVG edges carry source site, target site, evidence, and weight bindings.

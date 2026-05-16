# TERRAIN

Balanced sales and service territory planning.

TERRAIN is a Rust CLI and library for turning messy customer, job, workload,
and revenue data into compact, balanced territories with an auditable tradeoff
report. The first wedge is simple: upload or import sites, partition them into
territories, then explain balance, compactness, and workload gaps.

## First command

```powershell
cargo run -p terrain-cli -- sample-audit
cargo run -p terrain-cli -- sample-svg > terrain-split.svg
```

The scaffold command runs a built-in fixture and prints a small territory
balance audit. The SVG command emits a dashboard-ready visual split where each
territory and site carries stable `data-*` bindings for territory ID, site ID,
demand, revenue, assigned people, assignee counts, and dashboard joins.

## Product thesis

Most territory tools are either GIS-heavy dashboards or spreadsheets with
manual assignment rules. TERRAIN should make the hard part local, fast, and
auditable: graph construction, partitioning, compactness scoring, constraint
sweeps, scenario comparison, and dashboard-ready split visuals.

## Visual artifacts

TERRAIN should produce attractive, embeddable visual territory splits, not just
tables. The target artifacts are SVG first, then GeoJSON/TopoJSON once real
geography lands. Visual marks must be data-bound so users can reuse them in
their own dashboards instead of screenshotting the app.

## Non-goals

- TERRAIN does not own turn-by-turn routing.
- TERRAIN does not replace a CRM, dispatch system, or GIS.
- TERRAIN does not trap users in a proprietary dashboard; exports should be
  reusable in downstream BI and reporting tools.
- TERRAIN does not put customer-specific policy into shared kernels such as
  RLINE or METIS-CORE.

## License

MIT License. Copyright (c) Gio Della-Libera.

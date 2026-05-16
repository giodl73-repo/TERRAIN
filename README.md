# TERRAIN

Balanced sales and service territory planning.

TERRAIN is a Rust CLI and library for turning messy customer, job, workload,
and revenue data into compact, balanced territories with an auditable tradeoff
report. The first wedge is simple: upload or import sites, partition them into
territories, then explain balance, compactness, and workload gaps.

## First command

```powershell
cargo run -p terrain-cli -- sample-audit
```

The scaffold command runs a built-in fixture and prints a small territory
balance audit.

## Product thesis

Most territory tools are either GIS-heavy dashboards or spreadsheets with
manual assignment rules. TERRAIN should make the hard part local, fast, and
auditable: graph construction, partitioning, compactness scoring, constraint
sweeps, and scenario comparison.

## Non-goals

- TERRAIN does not own turn-by-turn routing.
- TERRAIN does not replace a CRM, dispatch system, or GIS.
- TERRAIN does not put customer-specific policy into shared kernels such as
  RLINE or METIS-CORE.

## License

MIT License. Copyright (c) Gio Della-Libera.

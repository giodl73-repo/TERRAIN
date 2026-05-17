# Phase 7: Public Land Survey - Edge Evidence

## Goal

Move TERRAIN from coordinate-only graph evidence toward explicit customer,
boundary, adjacency, and future geography-derived edge inputs.

## Thesis

Continental Divide proved that TERRAIN can build site graphs, diagnose them, and
hand stable CSR inputs to METIS-CORE. The next product step is evidence control:
operators should be able to say which site pairs are adjacent, connected, or
review-worthy before partitioning and dashboard handoff.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Site edge evidence CSV | done | Parsed explicit site-to-site edge rows and ran graph diagnostics from edge evidence. |
| 02 | Evidence-backed METIS handoff | planned | Build CSR handoff from explicit edge evidence instead of coordinate-complete graphs. |
| 03 | Edge evidence packet outputs | planned | Include graph evidence and diagnostics in review packets. |

## Success criteria

- Coordinate-derived graph behavior remains available as the default baseline.
- Explicit edge evidence can be parsed and diagnosed before partitioning.
- Unknown site references, duplicate edge rows, disconnected components, isolated
  sites, and long edges are visible to operators.
- METIS-CORE remains a graph engine dependency; TERRAIN owns edge evidence
  meaning and customer review language.

## Non-goals

- TERRAIN does not become a routing engine.
- TERRAIN does not own public geography acquisition.
- TERRAIN does not require GIS files before CSV edge evidence is useful.

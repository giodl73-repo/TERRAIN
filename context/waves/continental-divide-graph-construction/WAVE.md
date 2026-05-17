# Phase 6: Continental Divide - Graph Construction

## Goal

Build an explicit site graph layer so TERRAIN can move from coordinate-only
heuristics toward METIS-CORE-backed territory partitioning without moving
customer policy out of TERRAIN.

## Thesis

TERRAIN now has stable CSV intake, deterministic partitioning, scenario packets,
dashboard bindings, capacity review, and integration manifests. The next product
step is a graph contract: sites need stable node IDs, edge evidence, component
diagnostics, and a baseline comparison path before a shared partition engine is
worth adopting.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Site graph contract | done | Defined product-neutral graph nodes, edges, coordinate-distance weights, and diagnostics while preserving TERRAIN-owned territory policy. |
| 02 | Graph diagnostics CLI | done | Emits isolated-site, duplicate-coordinate, long-edge, and component diagnostics from site CSVs. |
| 03 | Graph-backed partition baseline | done | Added an internal graph-seeded partition report that compares against the existing greedy partition output. |
| 04 | METIS-CORE adoption adapter | done | Added a GitHub-backed METIS-CORE runtime adapter plus CSR handoff while keeping TERRAIN policy local. |

## Success criteria

- Existing CSV, packet, SVG, and GeoJSON commands continue to work.
- Graph construction is deterministic and validated with fixtures.
- Product-specific fairness, capacity, field-review language, and dashboard
  policy stay in TERRAIN.
- Any METIS-CORE dependency has a narrow graph-partitioning boundary and does
  not absorb TERRAIN policy.

## Completion

Continental Divide is complete. TERRAIN now has deterministic site graph
construction, graph diagnostics, a graph-seeded partition baseline, a
GitHub-backed METIS-CORE adapter, and CSR handoff artifacts for benchmark
comparison while keeping territory policy local.

## Non-goals

- TERRAIN does not become a turn-by-turn routing engine.
- TERRAIN does not own public geography acquisition; CROP/FLETCH remain future
  candidates for cached geography inputs.
- TERRAIN does not extract shared kernels until a second consumer or stable
  contract justifies it.


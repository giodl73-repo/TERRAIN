# TERRAIN Shared-Kernel Inventory

This inventory marks which TERRAIN mechanics are product policy and which are
candidates for shared kernels in RLINE or METIS-CORE.

## Boundary rules

- TERRAIN owns territory-planning policy: assignees, capacity, dashboards,
  review packets, and field-manager language.
- RLINE is the likely home for product-neutral tabular diagnostics, statistics,
  graph helpers, and dashboard schema primitives.
- METIS-CORE is the GitHub-backed runtime dependency for graph partitioning and
  refinement. TERRAIN owns the site-to-CSR adapter and product interpretation.
- CROP/PEBBLE/FLETCH are candidates for reusable cached geography, benchmark
  fixtures, and integration packets, not territory policy.

## Inventory

| TERRAIN surface | Current home | Candidate home | Extraction status | Notes |
|---|---|---|---|---|
| CSV line parsing and row diagnostics | `terrain-core` | RLINE `rdata`-style crate | candidate | Keep local until another repo needs the same diagnostics contract. |
| Spread ratios and centroid summaries | `terrain-core` | RLINE `rstat-core` / `rmath-core` | candidate | Product-neutral math, but TERRAIN field names stay local. |
| Compactness max-radius check | `terrain-core` | RLINE `rgraph-core` or `rmath-core` | candidate | Current implementation is coordinate-only, not a graph kernel yet. |
| Deterministic greedy partition | `terrain-core` | RLINE `ropt-core` | candidate | Useful as a baseline heuristic; not a METIS replacement. |
| Graph partition/refinement | METIS-CORE via GitHub dependency | METIS-CORE | active dependency | TERRAIN emits CSR handoff artifacts and audits METIS-backed partitions locally. |
| Movement manifests | `terrain-core` | TERRAIN | keep local | Site movement language is territory-review policy. |
| Capacity exceptions | `terrain-core` | TERRAIN | keep local | Fairness and workload thresholds are product policy. |
| SVG/GeoJSON data bindings | `terrain-core` | TERRAIN + documented schema | keep local | Field names are dashboard contract for this product. |
| Scenario/fairness packets | `terrain-cli` | TERRAIN | keep local | Operational handoff, not a reusable kernel. |

## Adoption sequence

1. Stabilize dashboard schemas and fixture outputs in TERRAIN.
2. Extract reusable CSV/diagnostic primitives only after a second repo needs
   the same row-context contract.
3. Keep the site graph and CSR adapter explicit in TERRAIN.
4. Use METIS-CORE through GitHub dependencies for graph partitioning.
5. Keep all people, capacity, fairness, and field-review wording in TERRAIN.

# TERRAIN Site Graph Contract

Contract ID: `terrain.site-graph.v1`

The site graph is TERRAIN's first explicit boundary between customer territory
policy and reusable graph mechanics. TERRAIN owns site meaning, territory
policy, capacity, fairness, field-review language, and dashboard bindings. The
graph contract only describes product-neutral nodes, edges, weights, and
diagnostics.

## Nodes

| Field | Meaning |
|---|---|
| `site_id` | Stable site identifier from TERRAIN intake. |
| `demand` | Numeric workload carried through from site intake. |
| `revenue` | Numeric revenue carried through from site intake. |
| `latitude` | Site latitude. |
| `longitude` | Site longitude. |

## Edges

| Field | Meaning |
|---|---|
| `from_site_id` | Stable source site ID. |
| `to_site_id` | Stable target site ID. |
| `weight` | Product-neutral edge cost. The first implementation uses coordinate distance in degrees. |
| `evidence` | Why the edge exists, e.g. `coordinate-distance-degrees` today and future adjacency or road evidence later. |

## Diagnostics

| Code | Severity | Meaning |
|---|---|---|
| `empty-site-set` | error | No sites were supplied for graph construction. |
| `duplicate-site-id` | error | More than one row has the same stable site ID. |
| `duplicate-coordinate` | warning | Multiple site IDs share the same coordinate. |
| `disconnected-component` | warning | A graph edge set contains more than one connected component. |
| `isolated-site` | warning | A graph node has no incident edges. |
| `long-edge` | warning | A coordinate-distance edge exceeds the review threshold. |

## CLI

```powershell
cargo run -p terrain-cli -- graph-diagnostics-csv sample-sites.csv
cargo run -p terrain-cli -- graph-partition-csv sample-sites.csv 2
cargo run -p terrain-cli -- metis-handoff-csv sample-sites.csv
cargo run -p terrain-cli -- metis-partition-csv sample-sites.csv 2
```

The diagnostics command emits a summary line and stable diagnostic rows:

```text
status=review node_count=6 edge_count=15 component_count=1 diagnostic_count=...
severity,code,site_ids,message
```

The partition command keeps `partition_sites` as the greedy baseline, builds the
site graph, creates a coordinate-graph-seeded partition, and emits comparison
rows for balance deltas, site movement, compactness exceptions, and graph
diagnostics. METIS-CORE adoption is handled through the narrow GitHub-backed
adapter below.

## METIS-CORE handoff

The first METIS-CORE integration is a narrow GitHub-backed runtime dependency
plus a fixture/benchmark handoff. `metis-handoff-csv` converts TERRAIN site graph
inputs into METIS-style CSR rows:

- `vertex_site_ids`: stable TERRAIN site IDs in deterministic vertex order,
- `xadj`: row pointers,
- `adjncy`: adjacency vertex IDs,
- `vwgt`: positive integer demand weights,
- `adjwgt`: positive integer coordinate-distance weights scaled by
  `edge_weight_scale`.

`metis-partition-csv` uses the same CSR handoff with METIS-CORE's Rust API and
audits the resulting territories through TERRAIN's existing balance report. This
keeps customer territory scoring, capacity, fairness, field-review language, and
dashboard bindings in TERRAIN while giving METIS-CORE a strict, product-neutral
input artifact for benchmark comparison.

## Boundary

Reusable candidates for RLINE or METIS-CORE:

- node/edge containers,
- coordinate-distance edge weights,
- connected-component diagnostics,
- graph partition input adapters.

TERRAIN-owned policy:

- territory labels,
- assignees and capacity,
- fairness exceptions,
- product demand interpretation,
- field-review language,
- dashboard schema field names.


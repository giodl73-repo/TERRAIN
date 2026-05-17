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
```

The command emits a summary line and stable diagnostic rows:

```text
status=review node_count=6 edge_count=15 component_count=1 diagnostic_count=...
severity,code,site_ids,message
```

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


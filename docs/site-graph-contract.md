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
| `weight` | Product-neutral edge cost. Coordinate-derived defaults use latitude-adjusted distance in degrees; explicit evidence rows may provide their own positive weights. |
| `evidence` | Why the edge exists, e.g. `latitude-adjusted-coordinate-distance-degrees`, `field-adjacency`, or future boundary/road evidence. |

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
cargo run -p terrain-cli -- graph-diagnostics-with-edges-csv sample-sites.csv sample-site-edges.csv
cargo run -p terrain-cli -- graph-partition-csv sample-sites.csv 2
cargo run -p terrain-cli -- metis-handoff-csv sample-sites.csv
cargo run -p terrain-cli -- metis-handoff-with-edges-csv sample-sites.csv sample-site-edges.csv
cargo run -p terrain-cli -- metis-partition-csv sample-sites.csv 2
cargo run -p terrain-cli -- metis-partition-with-edges-csv sample-sites.csv sample-site-edges.csv 2
cargo run -p terrain-cli -- edge-evidence-svg-csv sample-sites.csv sample-site-edges.csv > edge-evidence.svg
cargo run -p terrain-cli -- edge-evidence-geojson-csv sample-sites.csv sample-site-edges.csv > edge-evidence.geojson
cargo run -p terrain-cli -- edge-evidence-packet-csv sample-sites.csv sample-site-edges.csv terrain-edge-packet
cargo run -p terrain-cli -- territory-edge-audit-csv sample-territories.csv sample-site-edges.csv
cargo run -p terrain-cli -- territory-edge-packet-csv sample-territories.csv sample-site-edges.csv terrain-territory-edge-packet
```

The diagnostics command emits a summary line and stable diagnostic rows:

```text
status=review node_count=6 edge_count=15 component_count=1 diagnostic_count=...
severity,code,site_ids,message
```

`graph-diagnostics-with-edges-csv` uses explicit edge evidence rows with
`from_site_id`, `to_site_id`, `weight`, and `evidence`. It preserves the same
diagnostic report shape while surfacing unknown site references, self edges,
duplicate edges, disconnected components, isolated sites, and long edges from
operator-provided adjacency evidence.

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

The `*-with-edges-csv` METIS commands use explicit edge evidence instead of the
coordinate-complete graph. `edge-evidence-svg-csv` and
`edge-evidence-geojson-csv` render site-edge evidence with stable site, evidence,
and weight bindings. `edge-evidence-packet-csv` writes the supplied edge
evidence, graph diagnostics, METIS CSR handoff tables, SVG, and GeoJSON into a
review folder.

## Territory edge audit

`territory-edge-audit-csv` checks a territory plan against explicit edge
evidence. It reports:

- `cut-edge` when an edge connects sites assigned to different territories,
- `unknown-edge-site` when evidence references a site outside the plan,
- `disconnected-territory` when a multi-site territory has more than one
  connected component under the supplied edge evidence.

Cut edges are review signals, not automatic failures. A cut can be intentional
when capacity, ownership, or customer policy outweighs adjacency.

`territory-edge-field-review-csv` translates the audit into manager-ready
recommendations:

- `ready for field review` when no edge exceptions are present,
- `review adjacency exceptions with field managers` when cut edges or
  disconnected territories need a decision,
- `fix edge inputs before field review` when evidence references unknown sites.

`territory-edge-field-packet-csv` writes the field-review text, action CSV, raw
audit CSV, SVG, and GeoJSON together so the same site IDs and territory IDs can
be traced from a manager action back to the edge evidence.

## Manager exception register

`manager-exception-register-csv` combines existing review surfaces into one
operating-meeting register:

- balance warnings when the proposed plan exceeds demand or revenue spread
  thresholds,
- movement rows for sites that change territories,
- compactness warnings for territories above the review radius,
- capacity warnings for overloaded territories,
- edge actions from the edge field-review surface.

`manager-exception-packet-csv` writes the register CSV, summary text, proposed
territory SVG/GeoJSON, and edge evidence SVG/GeoJSON. The register is product
policy in TERRAIN; it does not replace source-specific graph, capacity, or
movement commands.

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


# TERRAIN

Balanced sales and service territory planning.

TERRAIN is a Rust CLI and library for turning messy customer, job, workload,
and revenue data into compact, balanced territories with an auditable tradeoff
report. The first wedge is simple: upload or import sites, partition them into
territories, then explain balance, compactness, and workload gaps.

## First command

Generate sample files first, then run the audit, graph, visual, and packet
commands against those files. Commands that read a CSV path also accept `-` for
stdin when piping from another tool.

```powershell
cargo run -p terrain-cli -- sample-audit
cargo run -p terrain-cli -- schema
cargo run -p terrain-cli -- integration-fixtures
cargo run -p terrain-cli -- integration-packet terrain-integration-packet
cargo run -p terrain-cli -- sample-svg > terrain-split.svg
cargo run -p terrain-cli -- sample-geojson > terrain-split.geojson
cargo run -p terrain-cli -- sample-csv > sample-territories.csv
cargo run -p terrain-cli -- sample-proposed-csv > sample-territories-proposed.csv
cargo run -p terrain-cli -- sample-sites-csv > sample-sites.csv
cargo run -p terrain-cli -- sample-site-edges-csv > sample-site-edges.csv
cargo run -p terrain-cli -- sample-capacity-csv > sample-capacity.csv
cargo run -p terrain-cli -- audit-csv sample-territories.csv
cargo run -p terrain-cli -- product-balance-csv sample-territories.csv
cargo run -p terrain-cli -- diagnose-csv sample-territories.csv
cargo run -p terrain-cli -- compare-csv sample-territories.csv sample-territories-proposed.csv
cargo run -p terrain-cli -- movement-csv sample-territories.csv sample-territories-proposed.csv
cargo run -p terrain-cli -- compactness-csv sample-territories.csv 0.06
cargo run -p terrain-cli -- packet-csv sample-territories.csv sample-territories-proposed.csv terrain-packet
cargo run -p terrain-cli -- field-review-csv sample-territories.csv sample-territories-proposed.csv
cargo run -p terrain-cli -- svg-csv sample-territories.csv > terrain-split.svg
cargo run -p terrain-cli -- geojson-csv sample-territories.csv > terrain-split.geojson
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
cargo run -p terrain-cli -- territory-edge-field-review-csv sample-territories.csv sample-site-edges.csv
cargo run -p terrain-cli -- territory-edge-field-packet-csv sample-territories.csv sample-site-edges.csv terrain-edge-field-packet
cargo run -p terrain-cli -- manager-exception-register-csv sample-territories.csv sample-territories-proposed.csv sample-capacity.csv sample-site-edges.csv
cargo run -p terrain-cli -- manager-exception-packet-csv sample-territories.csv sample-territories-proposed.csv sample-capacity.csv sample-site-edges.csv terrain-manager-exception-packet
cargo run -p terrain-cli -- capacity-csv sample-capacity.csv
cargo run -p terrain-cli -- capacity-audit-csv sample-territories.csv sample-capacity.csv
cargo run -p terrain-cli -- ownership-svg-csv sample-territories.csv sample-capacity.csv > terrain-ownership.svg
cargo run -p terrain-cli -- ownership-geojson-csv sample-territories.csv sample-capacity.csv > terrain-ownership.geojson
cargo run -p terrain-cli -- fairness-packet-csv sample-territories.csv sample-capacity.csv terrain-fairness-packet
cargo run -p terrain-cli -- partition-csv sample-sites.csv 2
cargo run -p terrain-cli -- sweep-csv sample-sites.csv 2 3
cargo run -p terrain-cli -- partition-svg-csv sample-sites.csv 2 > terrain-partition.svg
cargo run -p terrain-cli -- partition-geojson-csv sample-sites.csv 2 > terrain-partition.geojson
```

The scaffold command runs a built-in fixture and prints a small territory
balance audit. The CSV commands prove the first intake contract:
`territory_id`, `territory_label`, `assignees`, `site_id`, `demand`,
`revenue`, `latitude`, and `longitude`. The SVG and GeoJSON commands emit a
dashboard-ready visual split where each territory and site carries stable
`data-*` bindings for territory ID, site ID, demand, revenue, assigned people,
assignee counts, and dashboard joins.

The partition commands prove the first unassigned-site workflow: take site rows
with `site_id`, `demand`, `revenue`, `latitude`, and `longitude`, create a
deterministic initial split for the requested territory count, then audit or
render the result with the same dashboard-ready bindings in SVG or GeoJSON.

The scenario command starts the Lewis and Clark phase: compare a baseline plan
to a proposed plan and report pass/review status changes plus territory-level
site count, demand, and revenue deltas.

The diagnostics command checks messy territory CSV exports before parsing them
into a plan. It reports severity, line, field, and message for missing values,
duplicate site IDs, invalid numbers, and suspicious coordinates.

The compactness command reports territories whose max radius from centroid is
above a review threshold, giving early travel-proxy exceptions before full graph
or route kernels are adopted.

The packet command writes a review folder with scenario summary metrics,
per-territory deltas, movement manifest, diagnostics, compactness exceptions,
proposed SVG, and proposed GeoJSON for operations and dashboard handoff.

The movement command starts the Oregon Trail phase by listing site-level moves
between baseline and proposed plans with stable IDs, before/after territories,
movement kind, demand, and revenue.

The sweep command compares deterministic partitions across a target territory
count range, producing audit rows that can be sorted or charted.

The graph partition command compares the existing greedy partition baseline to a
coordinate-graph-seeded partition and reports balance deltas, site movement,
compactness exceptions, and graph diagnostics.

The METIS commands are the first adoption boundary: TERRAIN depends on
METIS-CORE through its GitHub repository, emits CSR-ready handoff rows for
benchmarking, and can audit a METIS-backed partition without moving territory
policy out of TERRAIN.

The territory edge audit command uses explicit edge evidence to flag site links
that cross territory boundaries, unknown edge references, and territories that
are disconnected by the supplied evidence.

The edge field review command turns those audit findings into manager-ready
recommendations and actions, and the packet command bundles the field-review
text, action rows, raw audit rows, SVG, and GeoJSON.

The manager exception register command combines balance, movement, compactness,
capacity, and edge findings into one triage CSV. Its packet command adds summary
text plus territory and edge visual artifacts.

The field review command emits a plain-language recommendation, balance summary,
site movement list, and compactness exceptions for field manager review.

The capacity commands start the Homestead Act phase by parsing assigned people
or team capacity, home-base coordinates, and skills for later fairness checks.
Capacity audit reports territories whose demand exceeds summed assigned-person
capacity.

Ownership visual commands add capacity, overload, owner count, and assignee
bindings to SVG and GeoJSON outputs for dashboard joins.
The fairness packet writes capacity roster, overload exceptions, ownership SVG,
and ownership GeoJSON into a review folder.

## Product thesis

Most territory tools are either GIS-heavy dashboards or spreadsheets with
manual assignment rules. TERRAIN should make the hard part local, fast, and
auditable: graph construction, partitioning, compactness scoring, constraint
sweeps, scenario comparison, and dashboard-ready split visuals.

## Development phases

TERRAIN uses American expansion/infrastructure-history motifs as planning
mnemonics:

1. **Louisiana Purchase** - product bounds, contracts, roles, and sample outputs.
2. **Lewis and Clark** - real data intake and exploration.
3. **Oregon Trail** - first movement/territory scenarios.
4. **Homestead Act** - people assignment, capacity, and ownership.
5. **Transcontinental Railroad** - shared kernels, dashboards, and integrations.
6. **Continental Divide** - explicit site graphs and graph-backed partition
   readiness.
7. **Public Land Survey** - explicit edge evidence, adjacency inputs, and
   geography-ready graph controls.
8. **National Road** - edge evidence SVG, GeoJSON, and packet review artifacts.
9. **Cumberland Gap** - edge-aware territory audit and review packet outputs.
10. **Pony Express** - plain-language edge field review and action packets.
11. **Chisholm Trail** - manager exception register across review surfaces.

## Visual artifacts

TERRAIN should produce attractive, embeddable visual territory splits, not just
tables. The target artifacts are SVG first, then GeoJSON/TopoJSON once real
geography lands. Visual marks must be data-bound so users can reuse them in
their own dashboards instead of screenshotting the app.

## Shared-kernel boundary

TERRAIN keeps territory policy local while tracking reusable candidates for
RLINE and METIS-CORE in `docs\shared-kernel-inventory.md`.
Dashboard bindings are tracked in `docs\dashboard-schema.md` and emitted with
`cargo run -p terrain-cli -- schema`.
Reusable fixture and cache-source handoffs are tracked in
`docs\integration-fixtures.md` and emitted with
`cargo run -p terrain-cli -- integration-fixtures`.
The integration packet command writes both manifests plus a policy-boundary
summary for downstream repos.

## Test scenarios

Richer test scenarios live under `fixtures\scenarios\` and are documented in
`docs\test-scenarios.md`. They cover steady-state balance, risky reassignment,
multi-product demand balance, multi-responsibility capacity, movement manifests,
field review, and growth sweeps.

## Non-goals

- TERRAIN does not own turn-by-turn routing.
- TERRAIN does not replace a CRM, dispatch system, or GIS.
- TERRAIN does not trap users in a proprietary dashboard; exports should be
  reusable in downstream BI and reporting tools.
- TERRAIN does not put customer-specific policy into shared kernels such as
  RLINE or METIS-CORE.

## License

MIT License. Copyright (c) Gio Della-Libera.

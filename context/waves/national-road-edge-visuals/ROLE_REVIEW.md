# TERRAIN Graph and Edge Evidence Role Review

Reviewed against the `.roles` panel after the Continental Divide, Public Land
Survey, and National Road waves.

## Parliament findings

| Role | Finding | Follow-up |
|---|---|---|
| Territory Planner | Graph and METIS outputs now expose balance, compactness, movement, connectedness, isolated sites, and edge evidence before partitioning. | Coordinate-derived graph weights now use latitude-adjusted longitude distance so east/west and north/south proximity are less distorted. |
| Operations Buyer | CSV, SVG, GeoJSON, and packet outputs fit spreadsheet and dashboard workflows, but the README first-command block needed clearer fixture ordering and pipeline support. | README now groups sample generation before dependent commands, and CSV-reading commands accept `-` for stdin. |
| Visual Split Designer | Edge evidence SVG makes operator-provided adjacency visible without turning TERRAIN into a routing product. | Keep edge visuals as review overlays and avoid adding route-style turn instructions. |
| Data Binding Auditor | Edge SVG and GeoJSON carry stable site IDs, evidence labels, weights, demand, and revenue. | Keep `data-from-site-id`, `data-to-site-id`, `evidence`, and `weight` stable unless a schema note is added first. |
| Fairness & Capacity Auditor | Graph waves did not disturb people/capacity commands, ownership visuals, or fairness packets. | Future graph-driven recommendations should continue to surface capacity and overload beside geography. |
| Kernel Boundary Engineer | METIS-CORE is a narrow GitHub dependency for graph partitioning; TERRAIN owns site meaning, edge evidence, review packets, and dashboard fields. | Continue depending through GitHub repos and keep customer policy out of METIS-CORE. |

## Editorial and stakeholder findings

| Role | Finding | Follow-up |
|---|---|---|
| README Product Editor | The first-command block had grown into a workflow and needed clearer sequencing. | Added guidance that sample files are generated first and CSV commands can read stdin with `-`. |
| Report Contract Editor | Edge packet outputs have explicit CSV headers and visual/GeoJSON fields with stable IDs and units. | Treat the edge visual fields as a report contract. |
| Field Manager | Edge evidence packets show which sites are connected without requiring graph vocabulary. | Keep field-review wording focused on sites, territories, workload, and exceptions. |
| Dashboard Builder | Edge SVG and GeoJSON can be joined by site IDs and embedded in downstream dashboards. | Future TopoJSON/geography work should preserve the same property names. |
| Future Agent | Phase and pulse docs are current through National Road. | Next wave should set a new active phase before implementation begins. |

## Issues found and addressed

1. README workflow sequencing was ambiguous once graph commands depended on
   generated fixture files. The README now calls out sample generation first.
2. File-only CSV intake limited command-line composability. CSV-reading commands
   now support `-` for stdin.
3. Coordinate-derived edge weights treated longitude degrees as equal to latitude
   degrees. Coordinate defaults now use a latitude-adjusted longitude delta and
   document the updated evidence label.

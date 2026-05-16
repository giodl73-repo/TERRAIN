# TERRAIN Foundation Role Review

Reviewed against the `.roles` panel after adding the dashboard-ready visual
split direction.

## Parliament findings

| Role | Finding | Plan change |
|---|---|---|
| Territory Planner | Balance must include operational review metrics, not just demand/revenue totals. | Phase 2 adds intake for travel proxy, compactness, exceptions, and scenario comparison. |
| Operations Buyer | The first paid wedge depends on outputs that fit existing operations workflows. | README and plan emphasize CSV/SVG/report artifacts that work beside spreadsheets, CRMs, and dashboards. |
| Visual Split Designer | TERRAIN should produce polished territory split visuals early. | `sample-svg` proves the first SVG split artifact and Phase 1 requires visual outputs. |
| Data Binding Auditor | Visuals must be reusable, not screenshots. | SVG marks expose `data-territory-id`, `data-site-id`, assignee counts, demand, revenue, and assignee names. |
| Fairness & Capacity Auditor | People assignment is part of territory truth. | Core summaries now include `assignee_count` and `assignees`; future phases add capacity flags. |
| Kernel Boundary Engineer | Product-specific territory policy should stay out of RLINE/METIS-CORE. | Phase 5 defers shared-kernel adoption until TERRAIN contracts stabilize. |

## Stakeholder findings

| Stakeholder | Finding | Plan change |
|---|---|---|
| Field Manager | A manager must see people, counts, workload, and exceptions without graph vocabulary. | Phase 4 focuses on people, ownership, capacity, and fairness. |
| Dashboard Builder | Exports need stable joins across visuals and rows. | Visual/dashboard contract requires shared IDs and units across CSV/SVG/JSON/GeoJSON. |
| Future Agent | Phase names should make the work memorable and resumable. | Phase map now uses Louisiana Purchase, Lewis and Clark, Oregon Trail, Homestead Act, and Transcontinental Railroad. |

## Phase naming rationale

The historical naming motif is used as a planning mnemonic:

1. **Louisiana Purchase** - define the product territory and boundaries.
2. **Lewis and Clark** - explore real input data and validate the model.
3. **Oregon Trail** - produce movement/scenario paths users can review.
4. **Homestead Act** - assign ownership, people, and capacity.
5. **Transcontinental Railroad** - connect kernels, exports, dashboards, and integrations.

These names are not intended to celebrate displacement or political harms; they
anchor product phases around survey, exploration, movement, ownership, and
connection.

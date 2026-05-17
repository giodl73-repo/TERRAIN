# Pony Express Role Review

Reviewed against the `.roles` panel after adding edge field-review language.

## Findings

| Role | Finding | Resolution |
|---|---|---|
| Territory Planner | Edge audit rows needed decision language, not only diagnostics. | `territory-edge-field-review-csv` recommends ready, manager review, or input fix status. |
| Operations Buyer | Review packets should tell the operating meeting what action to take. | `territory-edge-field-packet-csv` writes field-review text and action CSV alongside audit rows and visuals. |
| Visual Split Designer | Edge field review must preserve visual traceability without becoming routing. | Packet output keeps edge SVG/GeoJSON overlays and avoids turn-by-turn instructions. |
| Data Binding Auditor | Manager actions must join back to site IDs and territory IDs. | Action rows carry `territory_id`, `from_site_id`, and `to_site_id` fields. |
| Fairness & Capacity Auditor | A cut edge can be acceptable when capacity or ownership wins. | Review wording says to confirm or reassign, not automatically reject. |
| Kernel Boundary Engineer | Plain-language field review is TERRAIN policy. | METIS-CORE remains a graph/partition dependency; manager wording stays in TERRAIN. |

## Follow-up

Future waves can combine balance, capacity, movement, and edge findings into one
manager-ready exception register.

# Cumberland Gap Role Review

Reviewed against the `.roles` panel after adding edge-aware territory audits.

## Findings

| Role | Finding | Resolution |
|---|---|---|
| Territory Planner | A plan can look balanced while splitting important adjacency evidence across territories. | `territory-edge-audit-csv` reports `cut-edge` findings and disconnected territory components. |
| Operations Buyer | Edge audit output needs to fit review packets, not only algorithm logs. | `territory-edge-packet-csv` writes audit rows plus edge evidence SVG/GeoJSON artifacts. |
| Visual Split Designer | Edge audit findings should remain review overlays, not route maps. | Packet visuals reuse edge evidence SVG/GeoJSON and avoid turn-by-turn route semantics. |
| Data Binding Auditor | Audit rows must join back to site IDs and territory IDs. | CSV rows include `territory_id`, `from_site_id`, and `to_site_id`; visuals preserve site/evidence/weight bindings. |
| Fairness & Capacity Auditor | Cut edges should not override capacity or ownership rules. | Docs state cut edges are review signals, not automatic failures. |
| Kernel Boundary Engineer | Territory review semantics belong in TERRAIN, while graph mechanics stay reusable. | METIS-CORE remains a partition dependency; edge audit language and packet outputs stay local to TERRAIN. |

## Follow-up

Future waves should connect edge audit findings to field-review language so a
manager can understand why a cut edge is acceptable, risky, or needs reassignment.

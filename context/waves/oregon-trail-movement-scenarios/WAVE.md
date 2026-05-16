# Phase 3: Oregon Trail - Movement Scenarios

## Goal

Produce usable movement and territory split scenarios that field teams can
review before committing ownership changes.

## Thesis

After TERRAIN can compare and package scenarios, it should help users explore
how sites move between territories, where exceptions concentrate, and which
candidate splits are ready for human review.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Movement manifest | done | List site moves between baseline and proposed plans with stable IDs and before/after territories. |
| 02 | Scenario count sweep | done | Compare deterministic partitions across target territory counts. |
| 03 | Exception packet | pending | Combine movement, balance, compactness, and diagnostics into a review packet. |
| 04 | Field review export | pending | Emit a plain-language review artifact for field managers and dashboard builders. |

## Success criteria

- Site movement is explicit and traceable by stable site ID.
- Scenario sweeps are deterministic and comparable.
- Exceptions explain why a split needs review.
- Outputs remain usable in CSV, SVG, GeoJSON, and dashboard workflows.

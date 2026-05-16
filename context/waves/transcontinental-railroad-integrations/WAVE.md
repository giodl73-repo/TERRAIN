# Phase 5: Transcontinental Railroad - Shared Kernels and Integrations

## Goal

Connect TERRAIN to shared kernels, dashboard exports, and reusable integration
surfaces after the product contract has stabilized.

## Thesis

TERRAIN now has local contracts for intake, partitioning, movement, scenario
packets, and ownership fairness. The next phase should extract or adopt shared
mechanics only where the boundary is clear.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Shared-kernel inventory | done | Identify which local graph/stat/partition pieces should move to RLINE or METIS-CORE. |
| 02 | Dashboard schema contract | pending | Stabilize CSV, SVG, and GeoJSON field names for BI consumers. |
| 03 | Cached geography intake | pending | Evaluate CROP/PEBBLE/FLETCH inputs for reusable geography or benchmark fixtures. |
| 04 | Integration packet | pending | Package shared-kernel notes, schemas, fixtures, and visual exports for downstream repos. |

## Success criteria

- TERRAIN policy remains local.
- Reusable kernels have clear ownership candidates.
- Dashboard schemas stay stable and documented.
- Integration fixtures can be reused by future tools without customer policy.

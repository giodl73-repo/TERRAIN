# Phase 2: Lewis and Clark - Scenario Audit

## Goal

Explore real territory-plan inputs and compare proposed changes with clear,
auditable tradeoffs.

## Thesis

Once TERRAIN can ingest and visualize plans, the next useful artifact is a
before/after scenario report that explains whether a proposed split improves or
hurts balance, workload, revenue, and reviewability.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Scenario comparison | done | Compare baseline and proposed territory CSV plans with audit and per-territory deltas. |
| 02 | Intake diagnostics | done | Report missing fields, duplicate sites, invalid numbers, and suspicious coordinates without losing row context. |
| 03 | Compactness checks | pending | Surface simple compactness/travel-proxy exceptions for scenario review. |
| 04 | Scenario packet | pending | Export audit, deltas, SVG, and GeoJSON as a reviewable operations packet. |

## Success criteria

- Baseline and proposed plans can be compared from CSV.
- Scenario output explains pass/review status changes and territory-level deltas.
- Diagnostics preserve row context for messy customer/job data.
- Visual exports continue to share stable IDs with report rows.

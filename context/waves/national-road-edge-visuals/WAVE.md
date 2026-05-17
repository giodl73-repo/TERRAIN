# Phase 8: National Road - Edge Visuals

## Goal

Make explicit site-edge evidence reviewable as visual artifacts, not just rows.

## Thesis

Public Land Survey gave TERRAIN operator-provided adjacency evidence and
edge-backed METIS handoff. The next product step is visual trust: reviewers need
to see which site pairs are connected, why they are connected, and whether those
links line up with the sites they already review in SVG, GeoJSON, and packet
outputs.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Edge evidence SVG | done | Rendered explicit edge evidence as dashboard-bound SVG lines and site marks. |
| 02 | Edge evidence GeoJSON | done | Emitted edge evidence as GeoJSON LineString features plus site points. |
| 03 | Edge visual packet outputs | done | Included edge evidence SVG and GeoJSON in review packets. |

## Success criteria

- Edge visuals carry stable `data-*` / property bindings for site IDs, evidence,
  and weights.
- Edge visuals do not replace territory split visuals; they are review overlays.
- Existing edge diagnostics, METIS handoff, and METIS partition commands remain
  stable.

## Non-goals

- TERRAIN does not infer road routes or travel time.
- TERRAIN does not require external GIS files.
- TERRAIN does not move dashboard field naming into METIS-CORE.

## Completion

National Road is complete. TERRAIN can render explicit edge evidence as
dashboard-bound SVG, GeoJSON LineString features, and packet artifacts alongside
the existing edge diagnostics and METIS handoff tables.

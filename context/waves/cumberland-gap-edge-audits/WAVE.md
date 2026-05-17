# Phase 9: Cumberland Gap - Edge Audits

## Goal

Use explicit edge evidence to audit whether territory assignments respect
operator-provided adjacency and connectedness.

## Thesis

National Road made edge evidence visible. The next product step is territory
review: planners need to know when important edges are split across territories,
when a territory is internally disconnected by the supplied evidence, and when
edge evidence references sites that are not in the plan.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Territory edge audit CLI | done | Reported cut edges, unknown edge sites, and disconnected territory components. |
| 02 | Edge audit packet outputs | done | Wrote edge audit rows and visual artifacts into review packets. |
| 03 | Role review and docs | done | Documented edge audit semantics and review boundaries. |

## Success criteria

- Territory CSV outputs can be audited against site-edge evidence.
- Cut edges across territories and disconnected territory components are visible.
- Edge audit packet outputs remain useful without requiring graph vocabulary.
- TERRAIN owns territory review semantics; METIS-CORE stays a partition engine.

## Non-goals

- TERRAIN does not automatically reject every cut edge.
- TERRAIN does not infer roads or drive time.
- TERRAIN does not move field-review wording into METIS-CORE.

## Completion

Cumberland Gap is complete. TERRAIN can audit territory plans against explicit
edge evidence, report cut edges and disconnected territory components, and write
territory edge audit packets with edge rows and visual artifacts.

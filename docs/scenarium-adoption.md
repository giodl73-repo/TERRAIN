# SCENARIUM adoption

TERRAIN uses SCENARIUM only for product-neutral run identity, metric
comparison, structured review status, provenance, and evidence-packet closure.
Territory construction, balance thresholds, compactness, movement, capacity,
edge policy, field review, and all visual/report semantics remain in TERRAIN.

## Evidence slice

The retained risky-reassignment scenario compares:

- `fixtures/scenarios/steady-state-territories.csv`;
- `fixtures/scenarios/risky-reassignment-territories.csv`.

TERRAIN projects demand spread, revenue spread, and maximum radius as
lower-is-better metrics. A plan that exceeds TERRAIN's balance thresholds
receives the structured `terrain.balance.review` finding. The existing
`packet-csv` command retains every prior artifact and adds:

- `baseline-run.json`;
- `candidate-run.json`;
- `comparison.json`;
- `evidence-packet.json`.

All four documents use `scenarium.v1`. Repeated runs over the retained fixture
produce byte-identical twelve-file packets.

## Ownership review

| Lens | Finding |
|---|---|
| Territory Planner | Pass. Before/after territory deltas and existing operational reports remain TERRAIN-owned. |
| Operations Buyer | Pass. CSV, SVG, and GeoJSON artifacts remain usable without a SCENARIUM-specific UI. |
| Kernel Boundary Engineer | Pass with stop gate. SCENARIUM owns only neutral evidence contracts; no territory policy moved. |
| Report Contract Editor | Pass. `scenario-summary.csv` and all previous packet files remain; four versioned JSON files are additive. |

## Simplification result

The audit found no duplicate TERRAIN run, comparison, finding, provenance, or
evidence-packet type family to delete. `ScenarioComparison`, `BalanceAudit`,
and `TerritoryScenarioDelta` are domain reports rather than neutral contract
duplicates. Removing them would move territory semantics into SCENARIUM.

This disproves the proposed deletion target. The adoption is therefore bounded
to the additive evidence projection. Do not add shared TERRAIN/SCENARIUM glue,
RUNE descriptors, or further core APIs unless another named consumer exposes a
repeated neutral seam with measurable deletion.

## Dependency policy

TERRAIN pins SCENARIUM to immutable source commit
`f731ad4f99cc57479eafce6b68ea6521160c6853`. Portfolio policy prohibits crate
registry publication.

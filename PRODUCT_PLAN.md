# TERRAIN Product Plan

## Wedge

Balanced sales/service territories for field service, home health, pest
control, franchise operations, and sales teams.

## Phase naming motif

TERRAIN phases use famous American expansion and infrastructure-history moments
as mnemonics for product maturity. These names are product metaphors for survey,
movement, settlement, and connection work; they are not endorsements of the
historical harms or politics of expansion.

| Phase | Historical motif | Product meaning |
|---:|---|---|
| 1 | Louisiana Purchase | Define the territory domain: inputs, bounds, contracts, and review roles. |
| 2 | Lewis and Clark | Explore real customer/job data and validate the intake model. |
| 3 | Oregon Trail | Produce usable route/territory movement scenarios and visual handoffs. |
| 4 | Homestead Act | Assign people, capacity, and ownership to stable territories. |
| 5 | Transcontinental Railroad | Connect optimization kernels, dashboards, exports, and reusable integrations. |
| 6 | Continental Divide | Build explicit site graphs and prepare METIS-CORE-backed partitioning. |
| 7 | Public Land Survey | Add explicit edge evidence, adjacency inputs, and geography-ready graph controls. |
| 8 | National Road | Render edge evidence as reviewable SVG, GeoJSON, and packet artifacts. |
| 9 | Cumberland Gap | Audit territory plans against explicit edge evidence and connectedness. |
| 10 | Pony Express | Translate edge audit findings into field-manager actions and packets. |
| 11 | Chisholm Trail | Combine review findings into a manager exception register. |

## Phase 1: Louisiana Purchase - Foundation

Goal: establish the repo, first balance-audit contract, dashboard-ready visual
artifact direction, and wave/pulse workflow.

Pulses:

1. Workspace foundation: Rust workspace, docs, skills, and sample audit.
2. CSV intake: parse customer/job rows with workload, revenue, coordinates, and
   assigned people/team fields.
3. First partition: produce a deterministic balanced assignment and audit.
4. Visual split export: emit data-bound SVG, then GeoJSON/TopoJSON, with
   territory IDs, site IDs, assignees, counts, demand, revenue, and centroids.

Role review emphasis:

- Territory Planner: model must balance demand, revenue, compactness, and travel
  proxy together, not one metric at a time.
- Operations Buyer: outputs must show ROI/workflow fit before algorithms become
  complex.
- Visual Split Designer: visual territory splits are first-class artifacts.
- Data Binding Auditor: visuals must expose stable IDs and fields for dashboards.
- Fairness & Capacity Auditor: assigned people, counts, and capacity flags are
  part of the plan, not decoration.
- Kernel Boundary Engineer: keep product policy in TERRAIN; extract reusable
  graph/stat/partition mechanics only after contracts stabilize.

## Phase 2: Lewis and Clark - Scenario audit

Goal: compare territory plans with explainable tradeoffs.

Planned capabilities:

- CSV/JSON intake for customer/job rows, coordinates, demand, revenue, and
  assigned people/team fields.
- Balance metrics for demand, revenue, site count, people capacity, and travel
  proxy.
- Compactness and contiguity checks.
- Before/after scenario reports.
- Dashboard-ready SVG/GeoJSON outputs with stable data bindings.
- People/team assignment counts and capacity flags per territory.

## Phase 3: Oregon Trail - First movement scenarios

Goal: produce real territory split scenarios that can be reviewed by field
teams.

Planned capabilities:

- Deterministic initial assignment from customer/job rows.
- Scenario comparison across target territory counts.
- Exceptions list for outliers, disconnected clusters, and capacity overload.
- Exportable visual packets for operations review.

## Phase 4: Homestead Act - People and ownership

Goal: make every territory operationally owned and fair to the people assigned
there.

Planned capabilities:

- Assignee/team model with capacity, home base, skills, and workload limits.
- Per-person and per-team counts, demand, revenue, and route/travel proxy.
- Fairness/capacity flags surfaced in CSV, SVG, and future JSON/GeoJSON.

## Phase 5: Transcontinental Railroad - Shared-kernel adoption

Goal: move reusable graph and partition mechanics into shared dependencies.

Planned integrations:

- RLINE for shared graph/stat/optimization helpers.
- METIS-CORE for partition/refinement once the product contract is clear.
- CROP/PEBBLE/FLETCH later for context packs, import bundles, and cached public
  geography or benchmark fixtures.

## Phase 6: Continental Divide - Graph construction

Goal: move from coordinate-only territory heuristics to explicit site graph
construction while keeping customer policy in TERRAIN.

Planned capabilities:

- Build a product-neutral site graph contract from site IDs, coordinates, demand,
  revenue, and optional boundary/road adjacency inputs.
- Emit graph diagnostics before partitioning: isolated sites, duplicate
  coordinates, disconnected components, and suspect long edges.
- Keep deterministic greedy partitioning as the baseline comparator.
- Add a METIS-CORE adoption adapter only after the graph input contract is
  explicit and testable.
- Compare greedy and graph-backed partition outputs with the existing balance,
  movement, compactness, capacity, SVG, and GeoJSON surfaces.

## Phase 7: Public Land Survey - Edge evidence

Goal: let operators provide explicit edge evidence before geography imports or
road networks are available.

Planned capabilities:

- Parse site-to-site edge evidence with stable site IDs, weights, and evidence
  labels.
- Diagnose unknown site references, duplicate edge rows, disconnected
  components, isolated sites, and long edges.
- Feed explicit edge evidence into METIS-CORE handoff and review packets.
- Keep edge evidence meaning in TERRAIN while reusable graph mechanics stay in
  METIS-CORE.

## Phase 8: National Road - Edge visuals

Goal: make explicit graph evidence reviewable in the same dashboard and packet
surfaces as territory splits.

Planned capabilities:

- Render site-edge evidence as SVG line overlays with stable data bindings.
- Emit GeoJSON LineString features for edge evidence and point features for
  sites.
- Include edge visual artifacts in edge evidence packets.
- Keep edge visuals as review overlays, not turn-by-turn routes.

## Phase 9: Cumberland Gap - Edge audits

Goal: use explicit edge evidence to audit whether assigned territories preserve
important adjacency and connectedness.

Planned capabilities:

- Report site-edge evidence that crosses territory boundaries.
- Report territories that are internally disconnected by supplied edge evidence.
- Package edge audit rows with edge visuals for operations review.
- Keep cut-edge findings as review signals, not automatic rejection rules.

## Phase 10: Pony Express - Edge field review

Goal: turn edge audit findings into plain-language field-manager actions.

Planned capabilities:

- Recommend whether edge findings are ready for review, need manager review, or
  require input fixes.
- Translate cut edges, disconnected territories, and unknown edge sites into
  specific operating actions.
- Package field-review text, action rows, audit rows, and edge visuals together.
- Keep manager wording in TERRAIN while graph mechanics remain reusable.

## Phase 11: Chisholm Trail - Manager exception register

Goal: combine balance, movement, capacity, compactness, and edge findings into
one manager-ready exception register.

Planned capabilities:

- Emit one CSV register for operating-meeting triage.
- Keep stable territory and site IDs across balance, movement, capacity,
  compactness, and edge exceptions.
- Package the register with summary text and visual artifacts.
- Preserve each source surface while giving managers one prioritized review list.

## Visual/dashboard contract

TERRAIN visual outputs must be useful outside TERRAIN:

- **SVG first** for immediate dashboard embedding and customer-friendly review.
- **GeoJSON/TopoJSON later** for map dashboards and GIS interoperability.
- Every territory mark exposes `data-territory-id`, site counts, demand, revenue,
  assignee count, and assigned people.
- Every site mark exposes `data-site-id`, `data-territory-id`, demand, and
  revenue.
- Reports and visuals share IDs so Power BI, Tableau, Observable, custom web
  dashboards, and slide/report generators can bind rows to marks.

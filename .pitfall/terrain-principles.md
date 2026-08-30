# TERRAIN Principles

## TERRAIN-P-01: Territory Plans Are Tradeoff Reports

**Status:** ACTIVE

**Statement:** TERRAIN explains demand, revenue, site count, compactness,
movement, capacity, and edge tradeoffs instead of optimizing one hidden score.

**Decision rule:** New plan outputs must preserve enough metrics for a field
planner to see what improved and what got worse.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `.roles/parliament/territory-planner.md`,
and manager exception register tests.

## TERRAIN-P-02: Dashboard Bindings Are Product Contracts

**Status:** ACTIVE

**Statement:** CSV, SVG, GeoJSON, and packet outputs expose stable IDs and
fields so customers can join visual marks to operational rows.

**Decision rule:** New customer-facing exports must update
`terrain.dashboard.v1`, docs, tests, and CLI schema output together.

**Evidence:** `docs/dashboard-schema.md`, `.roles/parliament/data-binding-auditor.md`,
and `cargo run -p terrain-cli -- schema`.

## TERRAIN-P-03: People Capacity Is Not Decoration

**Status:** ACTIVE

**Statement:** A geography-balanced territory can still fail if assigned people
or teams are overloaded.

**Decision rule:** Capacity, ownership, assignees, and overload exceptions must
remain first-class in review packets and manager outputs.

**Evidence:** `.roles/parliament/fairness-capacity-auditor.md`,
`context/waves/homestead-act-ownership/WAVE.md`, and capacity tests.

## TERRAIN-P-04: Customer Policy Stays In TERRAIN

**Status:** ACTIVE

**Statement:** TERRAIN owns territory policy, field review wording, dashboard
fields, capacity thresholds, and customer-facing interpretation.

**Decision rule:** RLINE, METIS-CORE, SCENARIUM, MDCROP, MDPORT, and FLETCH may
provide bounded mechanics or fixtures, but product policy must not move into
shared kernels.

**Evidence:** `CLAUDE.md`, `docs/shared-kernel-inventory.md`,
`docs/scenarium-adoption.md`, and `.roles/parliament/kernel-boundary-engineer.md`.

## TERRAIN-P-05: Manager Review Needs One Register

**Status:** ACTIVE

**Statement:** Separate audit surfaces become useful to operators only when
they can be combined into one triage register with stable references.

**Decision rule:** New exception categories must preserve category, severity,
territory/site IDs, action text, and source-specific evidence.

**Evidence:** `context/waves/chisholm-trail-manager-exception-register/ROLE_REVIEW.md`,
`docs/site-graph-contract.md`, and manager exception register tests.

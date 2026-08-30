# TERRAIN Pitfalls

## TERRAIN-PF-01: Balanced Geography Hides People Overload

**Status:** MITIGATED

**Pattern:** Demand and compactness look balanced while one assignee, team, or
responsibility group carries unsustainable workload.

**Domain:** Capacity CSVs, ownership visuals, fairness packets, manager
exception registers, and customer review.

**Detection difficulty:** Geography and revenue summaries can pass while people
capacity remains invisible.

**Structural solution:** Keep capacity and overload as first-class fields,
packet artifacts, and manager exception rows.

**Evidence:** `.roles/parliament/fairness-capacity-auditor.md`,
`docs/dashboard-schema.md`, and `cargo test --workspace`.

## TERRAIN-PF-02: Dashboard Output Becomes Screenshot-Only

**Status:** MITIGATED

**Pattern:** SVG or GeoJSON output looks useful but lacks stable IDs and fields
needed for Power BI, Tableau, Observable, or custom dashboards.

**Domain:** SVG, GeoJSON, CSV reports, packet outputs, schema docs, and BI
handoffs.

**Detection difficulty:** Humans can inspect the visual even when downstream
tools cannot join marks back to rows.

**Structural solution:** Maintain `terrain.dashboard.v1` and test schema
coverage for customer-facing exports.

**Evidence:** `docs/dashboard-schema.md`, `.roles/parliament/data-binding-auditor.md`,
and schema tests.

## TERRAIN-PF-03: Shared Kernel Absorbs Customer Policy

**Status:** MITIGATED

**Pattern:** METIS-CORE, RLINE, or SCENARIUM starts owning capacity thresholds,
field review wording, dashboard fields, or territory interpretation.

**Domain:** Shared-kernel extraction, graph partitioning, evidence packets,
fixture handoffs, and dependency adoption.

**Detection difficulty:** Reuse pressure can make a helpful adapter look like a
new source of product truth.

**Structural solution:** Keep product policy in TERRAIN and export only bounded
mechanics or neutral evidence contracts.

**Evidence:** `docs/shared-kernel-inventory.md`, `docs/scenarium-adoption.md`,
and `.roles/parliament/kernel-boundary-engineer.md`.

## TERRAIN-PF-04: Edge Audit Becomes Automatic Rejection

**Status:** MITIGATED

**Pattern:** A cut edge or disconnected territory finding is treated as a
hard-fail even when field capacity, ownership, or customer policy may justify
the split.

**Domain:** Edge audit, field review, manager exception register, and customer
review packets.

**Detection difficulty:** Graph findings are concrete and can be overread as
decisions rather than review evidence.

**Structural solution:** Report edge findings as typed review actions and keep
manager language in TERRAIN.

**Evidence:** `docs/site-graph-contract.md`,
`context/waves/pony-express-edge-field-review/ROLE_REVIEW.md`, and
edge review tests.

## TERRAIN-PF-05: Manager Register Falls Out Of Schema

**Status:** MITIGATED

**Pattern:** A new combined manager exception register becomes the operating
meeting surface but is not listed in the dashboard schema.

**Domain:** Manager exception CSVs, packet outputs, dashboard schema, BI joins,
and customer handoff.

**Detection difficulty:** The register works as CSV, but downstream dashboard
builders lack a stable field contract.

**Structural solution:** Add `manager_exception_register` to
`terrain.dashboard.v1`, docs, and schema tests whenever the register surface is
customer-facing.

**Evidence:** PITFALL adoption updated `docs/dashboard-schema.md` and
`terrain-core/src/lib.rs` after the Chisholm Trail role review identified this
follow-up.

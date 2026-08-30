# TERRAIN Invariants

## TERRAIN-I-01: CSV Intake Keeps Stable Site And Territory IDs

**Status:** VERIFIED

**Invariant:** Territory and site rows preserve stable IDs through parsing,
audits, movement manifests, visuals, and packet outputs.

**Why it matters:** Customer review fails if visual marks, manager actions, and
CSV rows cannot be joined back to the same operational records.

**Test:** `cargo test --workspace` and README CSV command smokes.

**Evidence:** `README.md`, `docs/dashboard-schema.md`, and
`.roles/parliament/data-binding-auditor.md`.

## TERRAIN-I-02: Dashboard Schema Covers Customer-Facing Exports

**Status:** VERIFIED

**Invariant:** `terrain.dashboard.v1` names the fields exposed by reusable CSV,
SVG, GeoJSON, and packet surfaces, including the manager exception register.

**Why it matters:** Dashboard builders should consume a contract, not scrape
labels or infer columns from one sample file.

**Test:** `cargo test --workspace` and `cargo run -p terrain-cli -- schema`.

**Evidence:** `docs/dashboard-schema.md`, `terrain-core/src/lib.rs`, and
`context/waves/chisholm-trail-manager-exception-register/ROLE_REVIEW.md`.

## TERRAIN-I-03: METIS-CORE Handoff Is Product-Neutral

**Status:** VERIFIED

**Invariant:** TERRAIN emits CSR-ready graph rows for METIS-CORE while keeping
territory scoring, capacity, fairness, and field-review interpretation local.

**Why it matters:** Shared partition mechanics should not become the owner of
customer territory policy.

**Test:** `cargo test --workspace` plus METIS handoff and partition CLI smokes.

**Evidence:** `docs/site-graph-contract.md`, `docs/shared-kernel-inventory.md`,
and `.roles/parliament/kernel-boundary-engineer.md`.

## TERRAIN-I-04: SCENARIUM Projection Is Additive Evidence

**Status:** VERIFIED

**Invariant:** SCENARIUM packet files add neutral run identity and evidence
closure without replacing TERRAIN's domain reports.

**Why it matters:** Evidence-packet reuse is useful only while balance,
movement, compactness, and capacity semantics stay in TERRAIN.

**Test:** `cargo test --workspace` and `cargo run -p terrain-cli -- packet-csv`.

**Evidence:** `docs/scenarium-adoption.md` and scenario packet tests.

## TERRAIN-I-05: Edge Findings Are Review Signals

**Status:** VERIFIED

**Invariant:** Cut edges, disconnected territories, and unknown edge sites are
reported with stable evidence, but cut edges are not automatic rejection rules.

**Why it matters:** Field constraints can justify adjacency exceptions, while
bad edge inputs still need correction before review.

**Test:** `cargo test --workspace` and edge audit/field review CLI smokes.

**Evidence:** `docs/site-graph-contract.md`,
`context/waves/pony-express-edge-field-review/ROLE_REVIEW.md`, and
`context/waves/cumberland-gap-edge-audits/ROLE_REVIEW.md`.

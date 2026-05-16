# TERRAIN Product Plan

## Wedge

Balanced sales/service territories for field service, home health, pest
control, franchise operations, and sales teams.

## Wave 1: Foundation

Goal: establish the repo, first balance-audit contract, and wave/pulse workflow.

Pulses:

1. Workspace foundation: Rust workspace, docs, skills, and sample audit.
2. CSV intake: parse customer/job rows with workload, revenue, and coordinates.
3. First partition: produce a deterministic balanced assignment and audit.

## Wave 2: Scenario audit

Goal: compare territory plans with explainable tradeoffs.

Planned capabilities:

- Balance metrics for demand, revenue, site count, and travel proxy.
- Compactness and contiguity checks.
- Before/after scenario reports.

## Wave 3: Shared-kernel adoption

Goal: move reusable graph and partition mechanics into shared dependencies.

Planned integrations:

- RLINE for shared graph/stat/optimization helpers.
- METIS-CORE for partition/refinement once the product contract is clear.
- CROP/PEBBLE/FLETCH later for context packs, import bundles, and cached public
  geography or benchmark fixtures.

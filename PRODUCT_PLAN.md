# TERRAIN Product Plan

## Wedge

Balanced sales/service territories for field service, home health, pest
control, franchise operations, and sales teams.

## Wave 1: Foundation

Goal: establish the repo, first balance-audit contract, dashboard-ready visual
artifact direction, and wave/pulse workflow.

Pulses:

1. Workspace foundation: Rust workspace, docs, skills, and sample audit.
2. CSV intake: parse customer/job rows with workload, revenue, coordinates, and
   assigned people/team fields.
3. First partition: produce a deterministic balanced assignment and audit.
4. Visual split export: emit data-bound SVG, then GeoJSON/TopoJSON, with
   territory IDs, site IDs, assignees, counts, demand, revenue, and centroids.

## Wave 2: Scenario audit

Goal: compare territory plans with explainable tradeoffs.

Planned capabilities:

- Balance metrics for demand, revenue, site count, and travel proxy.
- Compactness and contiguity checks.
- Before/after scenario reports.
- Dashboard-ready SVG/GeoJSON outputs with stable data bindings.
- People/team assignment counts and capacity flags per territory.

## Wave 3: Shared-kernel adoption

Goal: move reusable graph and partition mechanics into shared dependencies.

Planned integrations:

- RLINE for shared graph/stat/optimization helpers.
- METIS-CORE for partition/refinement once the product contract is clear.
- CROP/PEBBLE/FLETCH later for context packs, import bundles, and cached public
  geography or benchmark fixtures.

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

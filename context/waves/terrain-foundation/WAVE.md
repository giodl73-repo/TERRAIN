# Phase 1: Louisiana Purchase - TERRAIN Foundation

## Goal

Create the first TERRAIN workspace and prove a minimal balance-audit plus
data-bound visual contract for territory plans.

## Thesis

The first useful artifact is an auditable territory plan plus a visual split
that can be dropped into dashboards. Mapping and partition refinement can land
after the model proves stable IDs, counts, assignees, demand, revenue, and
centroid bindings.

## Role review

| Role | Plan pressure | Resulting requirement |
|---|---|---|
| Territory Planner | A balanced table can still be a bad operational split. | Track demand, revenue, site count, compactness/travel proxy, and exceptions together. |
| Operations Buyer | The first wedge needs to show workflow and ROI, not algorithm spectacle. | Keep CSV/SVG/report outputs usable beside spreadsheets, CRM exports, and dashboards. |
| Visual Split Designer | Customers need a nice-looking split they can show. | SVG/GeoJSON exports are product artifacts, not debug output. |
| Data Binding Auditor | Screenshots are dead ends for dashboards. | Every territory/site visual mark carries stable IDs and metric bindings. |
| Fairness & Capacity Auditor | Territory balance can hide overloaded people. | Assigned people, assignee counts, and future capacity flags are core fields. |
| Kernel Boundary Engineer | Shared kernels should not absorb customer policy. | Keep policy in TERRAIN; only extract graph/stat/partition mechanics after contracts stabilize. |
| Field Manager | Field teams need plain explanations. | Output must show who owns each territory and why the split is reviewable. |
| Dashboard Builder | Downstream BI needs joins. | CSV/SVG/JSON/GeoJSON must share IDs and units. |

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Workspace foundation | done | Repo skeleton, docs, skills, sample balance audit, and sample SVG split. |
| 02 | CSV intake | done | Parse site/customer rows plus assigned people/team fields into the core audit model and render audit/SVG outputs from CSV. |
| 03 | First partition | pending | Produce a deterministic initial territory assignment and audit. |
| 04 | Visual export contract | pending | Emit dashboard-ready SVG/GeoJSON with shared data bindings. |

## Success criteria

- README explains the repo purpose and first command.
- Product plan names waves and non-goals.
- Wave/pulse scaffolding exists.
- Skills exist for future wave and pulse execution.
- Visual outputs carry stable data bindings for territory/site IDs, assigned
  people, counts, demand, revenue, and dashboard joins.
- Validation commands pass.

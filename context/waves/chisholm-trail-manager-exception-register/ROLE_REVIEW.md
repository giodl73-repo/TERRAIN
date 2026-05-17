# Chisholm Trail Role Review

Reviewed against the `.roles` panel after adding the manager exception register.

## Findings

| Role | Finding | Resolution |
|---|---|---|
| Territory Planner | Separate audit surfaces make it hard to see the full territory tradeoff. | `manager-exception-register-csv` combines balance, movement, compactness, capacity, and edge rows. |
| Operations Buyer | Operating meetings need one triage list, not several command outputs. | `manager-exception-packet-csv` writes one register plus summary and visuals. |
| Visual Split Designer | Exception rows still need visual context. | Packet output includes proposed territory SVG/GeoJSON and edge SVG/GeoJSON. |
| Data Binding Auditor | Rows must remain dashboard-joinable. | Register rows carry category, severity, territory ID, site ID, and edge endpoint fields. |
| Fairness & Capacity Auditor | Capacity overloads should be visible beside movement and edge risks. | Capacity exceptions are first-class register rows. |
| Kernel Boundary Engineer | The combined register is product policy, not graph infrastructure. | The register stays in TERRAIN and reuses existing local review surfaces. |

## Follow-up

Future waves can add dashboard schema coverage for the manager exception register
so downstream BI tools can bind the triage CSV directly to visual marks.

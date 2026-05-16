# TERRAIN Test Scenarios

TERRAIN has small smoke fixtures in `fixtures\`, plus richer scenario fixtures
in `fixtures\scenarios\` for exercising customer-style workflows.

## Scenario pack

| Scenario | Files | What it tests |
|---|---|---|
| Steady state | `steady-state-territories.csv`, `steady-state-capacity.csv` | Balanced three-territory plan with enough owner capacity and five product-demand lanes. |
| Risky reassignment | `steady-state-territories.csv`, `risky-reassignment-territories.csv`, `steady-state-capacity.csv` | Site movement, demand imbalance, product-demand imbalance, south capacity overload, and compactness review. |
| Growth sweep | `growth-sites.csv` | Deterministic partitions across target territory counts for expansion planning with mixed product curves. |

## Useful commands

```powershell
cargo run -p terrain-cli -- audit-csv fixtures\scenarios\steady-state-territories.csv
cargo run -p terrain-cli -- compare-csv fixtures\scenarios\steady-state-territories.csv fixtures\scenarios\risky-reassignment-territories.csv
cargo run -p terrain-cli -- product-balance-csv fixtures\scenarios\risky-reassignment-territories.csv
cargo run -p terrain-cli -- movement-csv fixtures\scenarios\steady-state-territories.csv fixtures\scenarios\risky-reassignment-territories.csv
cargo run -p terrain-cli -- capacity-audit-csv fixtures\scenarios\risky-reassignment-territories.csv fixtures\scenarios\steady-state-capacity.csv
cargo run -p terrain-cli -- field-review-csv fixtures\scenarios\steady-state-territories.csv fixtures\scenarios\risky-reassignment-territories.csv
cargo run -p terrain-cli -- sweep-csv fixtures\scenarios\growth-sites.csv 2 4
```

The scenario capacity fixture includes multiple responsibility and asset lanes
per person, so tests cover employees who split time across products, assets, or
work types.

These fixtures are intentionally synthetic. They test behavior and dashboard
contracts without embedding customer-specific policy or private data.

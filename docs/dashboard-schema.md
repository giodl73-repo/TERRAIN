# TERRAIN Dashboard Schema

Schema ID: `terrain.dashboard.v1`

The schema is the stable field contract shared by CSV reports, SVG `data-*`
bindings, GeoJSON properties, and packet outputs.

## Exports

| Export | Stable fields |
|---|---|
| `territory` | `territory_id`, `territory_label`, `site_count`, `demand`, `revenue`, `capacity`, `overload`, `owner_count`, `assignee_count`, `assignees`, `centroid_latitude`, `centroid_longitude`, `max_radius_degrees` |
| `site` | `site_id`, `territory_id`, `territory_label`, `demand`, `revenue`, `capacity`, `overload`, `owner_count`, `assignee_count`, `assignees`, `latitude`, `longitude` |
| `scenario_delta` | `territory_id`, `baseline_site_count`, `proposed_site_count`, `site_count_delta`, `baseline_demand`, `proposed_demand`, `demand_delta`, `baseline_revenue`, `proposed_revenue`, `revenue_delta` |
| `movement` | `site_id`, `baseline_territory_id`, `proposed_territory_id`, `movement_kind`, `demand`, `revenue` |
| `capacity_exception` | `territory_id`, `demand`, `capacity`, `overload`, `assignees` |

## CLI

```powershell
cargo run -p terrain-cli -- schema
```

The command emits the machine-readable `terrain.dashboard.v1` contract.

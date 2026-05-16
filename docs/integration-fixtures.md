# TERRAIN Integration Fixtures

Manifest ID: `terrain.integration-fixtures.v1`

This manifest identifies reusable fixture and cache-source handoffs without
making TERRAIN depend on CROP, PEBBLE, or FLETCH yet.

## Candidate sources

| Source | Repo | Role | Status |
|---|---|---|---|
| `crop-geography-cache` | CROP | Cached public geography and boundaries | candidate |
| `pebble-context-packets` | PEBBLE | Portable context and benchmark packets | candidate |
| `fletch-fetch-cache` | FLETCH | Registered URL/cacheline-backed fixture retrieval | candidate |

## Current TERRAIN fixtures

| Fixture | Path | Role |
|---|---|---|
| `sample-territories` | `fixtures\sample-territories.csv` | Assigned territory baseline |
| `sample-sites` | `fixtures\sample-sites.csv` | Unassigned partition input |
| `sample-capacity` | `fixtures\sample-capacity.csv` | Assignee capacity input |

## CLI

```powershell
cargo run -p terrain-cli -- integration-fixtures
```

The command emits the machine-readable `terrain.integration-fixtures.v1`
manifest for downstream tools and future cache registration work.

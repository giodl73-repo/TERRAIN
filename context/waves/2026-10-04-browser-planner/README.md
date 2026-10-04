# Browser territory planner

Status: implementation started.

Expose the existing TERRAIN parsing, partitioning, balance audit, SVG, and
GeoJSON functions through a bounded Rust/WASM adapter. Start with a small public
synthetic dataset; allow local CSV input. Show demand/revenue balance, assignments,
and coordinate-radius proxies honestly. No road-travel-time or polygon-boundary
claim. Processing stays in a worker and uploaded files remain on the device.

Gates: existing/native adapter tests, fmt/clippy, release WASM, browser controls,
CSV error/recovery, exports, mobile layout, bundle budget, clean review, hosted CI,
and exact default-branch/live verification before the TRACKER snapshot.

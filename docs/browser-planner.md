# Browser territory planner

The browser calls `terrain-web::plan` in a worker. That adapter validates bounded
CSV, reuses native parsing and greedy/METIS partitioning, and exports native SVG
and GeoJSON. No JavaScript copy of the planner or server upload is involved.

Demand and revenue ratios mean maximum territory total / minimum total: 1 is
balanced. Equal zero totals count as balanced; zero versus positive totals has
an unbounded ratio and fails finite limits. The adapter derives these ratios
from native audit summaries because the core audit exposes a different spread
metric. Coordinate radius is measured in degrees, not road travel time.

The 18-site sample is synthetic. A shared URL contains only settings and always
loads that sample; private CSV is neither placed in the URL nor transmitted.
SVG is displayed as an inert image. Invalid inputs clear results and disable
exports until a successful calculation. Inputs are capped at 500 KB / 2,000 sites.

Build with `python tools/build-pages.py` after installing the Rust wasm target
and wasm-bindgen-cli 0.2.127. `npm ci` and `npx playwright test` exercise real WASM,
keyboard controls, both methods, CSV recovery, download, mobile width, and load
failure. CI runs native workspace tests, adapter clippy, release WASM, browser
checks, and a 5 MB output gate before deployment on main.

Publication receipts are recorded in GitHub Actions and the TRACKER wave.

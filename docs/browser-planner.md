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

Build with `python tools/build-pages.py --inputs ../terrain-inputs --cargo-home ../terrain-cargo-home`
after installing the Rust wasm target and wasm-bindgen-cli 0.2.127; the
[verified input kit](../cargo-inputs/README.md) creates or verifies those new
owned offline sources. `npm ci` and `npx playwright test` exercise real WASM,
keyboard controls, both methods, CSV recovery, download, mobile width, and load
failure. CI runs native workspace tests, adapter clippy, release WASM, browser
checks, and a 5 MB output gate before deployment on main.

Publication receipts are recorded in GitHub Actions and the TRACKER wave.

## Worker lifetime and current results

Changing CSV or settings immediately clears the map, metrics, table and export.
A completed result belongs to the exact submitted input snapshot; a newer input
cannot export an older result. At most one current worker and one pending plan
are retained. Replacing active work terminates that worker; generation and
request checks reject its late ready, error and result messages.

Stop planning terminates initialization or computation and keeps local CSV and
settings. Retry planner starts a fresh worker. Engine initialization has a
15-second deadline; computation has a two-minute deadline enforced by the main
thread, including when a worker is blocked. The public sample request has its
own 10-second abort deadline and bounded body. Sample failure leaves local CSV
usable; Load sample retries it. Late sample/file reads cannot replace newer
input, and initialization failure cannot revive a queued automatic sample plan.

A measured 2,000-site graph case takes about 37 seconds in installed Node/WASM
on the observing Windows laptop; greedy takes about 33 ms. These observations
set a bounded recovery policy, not a promise about mobile hardware or workload
speed. The full admitted case is also tested in the real browser with all 2,008
GeoJSON site/territory features retained. Fault-injection tests deliberately
stall workers and accelerate only main-thread clocks; successful planning still
executes the actual Rust WASM. CSV never appears in requests or shared URLs.

Web files are LF-pinned through .gitattributes so generated runtime bytes and
sample input agree with raw Git/source-archive bytes across checkout platforms.
Earlier Windows CRLF and hosted LF controls remain distinct historical records;
no output normalization is used to claim exact native/WASM equality.

# Original locked Rust inputs

`inputs.zip` contains the 25 complete original registry archives and both
immutable Git file trees named by `Cargo.lock`. It preserves all 1,150 original
files, including files that `cargo vendor` filters out, and the original Git
manifests. Git commit/tree/blob identities, source modes, registry checksums and
all original grant/notice inventories are verified before restoration.

The current capsule is 2,581,467 bytes. Its SHA-256 is
`a937b404bffb49b2433cbf8f6887dc45a238a64f44ef60be55a750fc682a83dc`.
The SHA in `inputs.sha256` is useful after independently verifying the source
checkout/archive; a checksum supplied alongside an untrusted download does not
establish authorship. Input verification/restoration needs only Python 3.11+
standard library, without Git, Cargo, network or installed Python packages.

```powershell
python -I -S -B tools/input_custody.py --sha256 a937b404bffb49b2433cbf8f6887dc45a238a64f44ef60be55a750fc682a83dc --inputs ../terrain-inputs --cargo-home ../terrain-cargo-home
```

Use two new, separate folders outside this source checkout and all compiled
caches. Existing folders, links/junctions, unsafe archives and overlapping
destinations are rejected before writes. `--protect-target PATH` adds any
compiled cache not already named by `CARGO_TARGET_DIR` or the source's default
`target` path. Restoration never removes existing data or silently falls back
to a default Cargo cache. Use `--readback` with the same arguments to check the
complete restored source/config inventory after compilation.

Build with the installed Rust 1.95.0 toolchain, WASM target and wasm-bindgen CLI
0.2.127, plus the platform linker needed for native tests:

```powershell
python -B tools/build-pages.py --inputs ../terrain-inputs --cargo-home ../terrain-cargo-home --target ../terrain-target
```

The builder can also create a new pair of input/home folders directly. It
verifies an existing pair before reuse, forces Cargo offline, checks that every
locked dependency resolves under those owned sources, remaps source/dependency/
target diagnostic paths and checks original inputs again after compilation.
Compiled targets can be reused or explicitly new; this is separate from input
custody. The installed compiler/linker/bindgen, Node and browser test tools are
not redistributed in the capsule. Browser QA uses the versions in
`package-lock.json`; Node/Playwright/Chromium remain explicit prerequisites.

Original package grants remain scoped to their own sources. METIS-CORE retains
Apache-2.0 and upstream NOTICE; historical MIT observations do not supersede
those upstream-derived terms. The verifier derives a complete original notice
concatenation for publication without rewriting any notice text.

For maintainers updating the lock, `python -B tools/make-inputs.py --output NEW.zip`
authors a **new** capsule from already installed original archives/Git objects,
then independently verifies it before writing. It does not acquire packages,
rewrite Cargo manifests or overwrite existing output. Review the new SHA,
original source/notices and the complete delivery budget before replacing a kit.

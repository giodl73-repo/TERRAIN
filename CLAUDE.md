# TERRAIN - Claude Code Context

Project: TERRAIN balanced territory planner.

Working directory: `C:\src\TERRAIN`.

## Rules

- Keep customer/product policy in TERRAIN.
- Keep reusable graph, stat, and partition kernels in RLINE or METIS-CORE after
  the contract stabilizes.
- Prefer small pulses under `context/waves/`.
- Validate with:

```powershell
cargo fmt --check
cargo test --workspace
cargo run -p terrain-cli -- sample-audit
cargo run -p terrain-cli -- sample-svg
git diff --check
```

## Git identity

Commits use:

- `user.name = giodl73-repo`
- `user.email = giodl73@gmail.com`

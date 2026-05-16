# Pulse 01: Shared-Kernel Inventory

Status: done

## Goal

Identify which local TERRAIN mechanics should stay as product policy and which
should become candidates for RLINE, METIS-CORE, or cached-data integrations.

## Outcome

- Added `docs\shared-kernel-inventory.md`.
- Marked CSV diagnostics, statistics, compactness, and greedy partitioning as
  candidates for RLINE-style shared primitives.
- Marked future graph partition/refinement as a METIS-CORE integration once
  TERRAIN has stable graph construction.
- Kept movement, capacity, ownership, visual bindings, and review packets local
  to TERRAIN policy.

## Validation

- `cargo fmt --check`
- `cargo test --workspace`
- `git diff --check`

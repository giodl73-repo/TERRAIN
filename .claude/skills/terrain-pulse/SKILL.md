---
name: terrain-pulse
description: Execute the next TERRAIN wave pulse with docs, implementation, validation, and commit-ready updates.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# TERRAIN Pulse

Use this skill for TERRAIN development pulses.

## Workflow

1. Read `context/waves/PHASES.md`.
2. Read the active wave `WAVE.md`.
3. Read the target pulse under `pulses/`.
4. Implement the smallest complete slice.
5. Keep shared kernels free of TERRAIN-specific behavior.
6. Update docs and wave/pulse status.
7. Run validation commands and `git diff --check`.

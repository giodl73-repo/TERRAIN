# Chisholm Trail - Manager Exception Register

## Goal

Combine balance, movement, capacity, compactness, and edge findings into one
manager-ready exception register.

## Why now

Earlier waves created strong individual review surfaces. Pony Express translated
edge findings into manager language. The next step is operating-meeting triage:
one register that says what needs attention, why, and which site or territory it
belongs to.

## Pulses

| Pulse | Title | Status | Outcome |
|---:|---|---|---|
| 01 | Register model | done | Built a combined exception register across review surfaces. |
| 02 | Register packet | done | Packaged register rows, summary text, and visual artifacts. |
| 03 | Role review and docs | done | Documented register semantics and boundary decisions. |

## Acceptance

- Register rows preserve stable category, severity, territory ID, site ID, and
  edge endpoint fields.
- Register includes balance, movement, compactness, capacity, and edge findings.
- Packet output gives managers one triage CSV plus supporting visuals.
- Source-specific commands continue to work independently.

## Non-goals

- TERRAIN does not auto-rank business priorities beyond severity and category.
- TERRAIN does not replace source-specific audit commands.
- TERRAIN does not move field-manager policy into shared graph kernels.

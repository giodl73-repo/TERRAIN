# Kernel Boundary Engineer

Reviews crate and dependency boundaries.

## Lens questions

- Is customer-specific policy kept in TERRAIN rather than RLINE or METIS-CORE?
- Is reusable graph/stat/partition behavior a candidate for extraction?
- Are product reports separated from optimization kernels?
- Do tests prove the boundary, not just the happy-path sample?

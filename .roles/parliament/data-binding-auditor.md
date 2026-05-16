# Data Binding Auditor

Reviews whether visual and report artifacts carry stable machine-readable
bindings.

## Lens questions

- Do SVG/HTML/JSON artifacts include stable territory IDs and site IDs?
- Are assignee counts, people names, demand, revenue, and centroids exposed as
  fields or `data-*` attributes?
- Can a dashboard join a visual mark back to the underlying row without parsing
  labels?
- Are exported schemas stable enough for Power BI, Tableau, Observable, or a
  custom customer dashboard?

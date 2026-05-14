

## WORKING MEMORY
[2026-05-14T08:56:42.048Z] LineChart fix: replaced broken quadratic smoothing with Catmull-Rom cubic Bézier path helper; added smooth_area_path sharing exact top boundary with line; changed area gradient to vertical 180deg top-color to bottom-transparent. Verified cargo fmt/check/test and GUI smoke timeouts.

[2026-05-14T09:13:06.218Z] Pie/Ring antialias fix: replaced polygonal arc approximation (PIE_STEPS_PER_SLICE + arc_points) with GPUI PathBuilder arc_to-based native circular arcs. Full circle/hole uses two 180deg arcs; slices use exact arc_to from start to end. Verified fmt/check/test/smoke.


## WORKING MEMORY
[2026-05-14T08:56:42.048Z] LineChart fix: replaced broken quadratic smoothing with Catmull-Rom cubic Bézier path helper; added smooth_area_path sharing exact top boundary with line; changed area gradient to vertical 180deg top-color to bottom-transparent. Verified cargo fmt/check/test and GUI smoke timeouts.

[2026-05-14T09:13:06.218Z] Pie/Ring antialias fix: replaced polygonal arc approximation (PIE_STEPS_PER_SLICE + arc_points) with GPUI PathBuilder arc_to-based native circular arcs. Full circle/hole uses two 180deg arcs; slices use exact arc_to from start to end. Verified fmt/check/test/smoke.
[2026-05-14T09:29:25.258Z] Chart value indicators: added ChartOptions.show_value_labels default true with builder toggles for LineChart/AreaChart/BarChart; PieChart/RingChart have show_value_labels default true. Labels are painted natively via GPUI shaped text on canvas; pie/ring labels show value + percent. Verified full workspace checks/tests and GUI smoke.
[2026-05-14T09:46:11.942Z] Pie/Ring labels corrected: label format is value / total, optional percentage defaults on with configurable percentage_decimals (clamped <=4), small slices below outside_threshold_degrees route to outside leader-line labels. Demos/snippets now include small slices to exercise callouts. Verified full workspace checks/tests and GUI smoke.
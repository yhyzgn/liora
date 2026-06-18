# Liora Logo Candidates

These files are alternate main-library Liora brand marks. They intentionally do
not replace the current `packaging/icons/liora.*` assets yet.

Candidate set:

- `liora-candidate-orbit.*` — abstract orbital core, no letterform.
- `liora-candidate-prism.*` — 3D component/system prism.
- `liora-candidate-ribbon.*` — flowing aurora ribbon mark.
- `liora-candidate-monolith.*` — document/component slab with liora arc.

Each candidate includes:

- `.svg` editable source.
- `.png` 1024x1024 preview/package source.
- `.ico` multi-size Windows icon.
- `.icns` multi-size macOS icon.

Once a candidate is selected, copy its SVG/PNG/ICO/ICNS files over
`packaging/icons/liora.*` and rerun `cargo xtask package validate`.

from pathlib import Path
import re
import shutil
import sys

source = Path(sys.argv[1])
dest = Path(sys.argv[2])
full = len(sys.argv) > 3 and sys.argv[3] == "--full"
ICON_SET = "tabler"

def pascal(value: str) -> str:
    out = "".join((part[:1].upper() + part[1:]) for part in re.split(r"[^A-Za-z0-9]+", value) if part)
    if not out:
        out = "Icon"
    if out[0].isdigit():
        out = "I" + out
    return out

def kebab_from_variant(value: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "-", value).lower() + ".svg"

def collect_entries():
    entries = []
    if ICON_SET == "antd":
        for path in sorted(source.glob("*/*.svg")):
            suffix = {"filled": "Filled", "outlined": "Outlined", "twotone": "Twotone"}.get(path.parent.name, pascal(path.parent.name))
            entries.append((pascal(path.stem) + suffix, path))
    elif ICON_SET == "ionic":
        for path in sorted(source.glob("*.svg")):
            stem = path.stem
            suffix = ""
            if stem.endswith("-outline"):
                stem = stem[:-8]
                suffix = "Outline"
            elif stem.endswith("-sharp"):
                stem = stem[:-6]
                suffix = "Sharp"
            entries.append((pascal(stem) + suffix, path))
    elif ICON_SET == "tabler":
        for path in sorted((source / "outline").glob("*.svg")):
            entries.append((pascal(path.stem), path))
        for path in sorted((source / "filled").glob("*.svg")):
            entries.append((pascal(path.stem) + "Filled", path))
    elif ICON_SET == "carbon":
        candidates = {}
        for path in sorted(source.glob("*/*.svg")):
            try:
                size = int(path.parent.name)
            except ValueError:
                continue
            candidates.setdefault(path.stem, []).append((size, path))
        for stem, values in sorted(candidates.items()):
            values.sort(key=lambda item: ({32: 0, 24: 1, 20: 2, 16: 3}.get(item[0], 100 - item[0]), item[0]))
            entries.append((pascal(stem), values[0][1]))
    elif ICON_SET == "material":
        suffixes = {"materialicons": "", "materialiconsoutlined": "Outlined", "materialiconsround": "Round", "materialiconssharp": "Sharp", "materialiconstwotone": "Twotone"}
        best = {}
        for path in source.glob("*/*/materialicons*/**/*.svg"):
            parts = path.relative_to(source).parts
            if len(parts) != 4:
                continue
            _category, icon, style, file_name = parts
            if style not in suffixes:
                continue
            size = file_name.removesuffix("px.svg")
            if size not in ("24", "20"):
                continue
            key = (icon, style)
            current = best.get(key)
            if current is None or (current[0] != "24" and size == "24"):
                best[key] = (size, path)
        for (icon, style), (_size, path) in sorted(best.items()):
            entries.append((pascal(icon) + suffixes[style], path))
    else:
        raise SystemExit(f"unknown icon set: {ICON_SET}")
    return entries

dest.mkdir(parents=True, exist_ok=True)
seen_variants = set()
seen_files = set()
added = updated = skipped = 0
for variant, path in collect_entries():
    base_variant = variant
    counter = 2
    while variant in seen_variants:
        variant = f"{base_variant}{counter}"
        counter += 1
    seen_variants.add(variant)
    file_name = kebab_from_variant(variant)
    base_file = file_name
    counter = 2
    while file_name in seen_files:
        file_name = base_file[:-4] + f"-{counter}.svg"
        counter += 1
    seen_files.add(file_name)
    target = dest / file_name
    data = path.read_bytes()
    if not target.exists():
        target.write_bytes(data)
        added += 1
    elif full or target.read_bytes() != data:
        target.write_bytes(data)
        updated += 1
    else:
        skipped += 1
print(f"{ICON_SET}: +{added} added, ~{updated} updated, ={skipped} skipped, total={len(seen_files)}")

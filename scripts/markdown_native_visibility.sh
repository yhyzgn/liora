#!/usr/bin/env bash
# Run with bash scripts/markdown_native_visibility.sh; owns an isolated Xwayland display and clipboard.
set -euo pipefail
: "${WAYLAND_DISPLAY:?Set WAYLAND_DISPLAY to the host compositor socket for isolated Xwayland}"
cd "$(dirname "$0")/.."
for command in Xwayland magick import; do command -v "$command" >/dev/null; done
cargo build -p liora-gallery --example markdown_editing_smoke --offline
proof=$(mktemp -d /tmp/liora-markdown-visibility.XXXXXX)
app=
display_pid=
cleanup() {
    if [[ -n "$app" ]]; then kill "$app" 2>/dev/null || :; wait "$app" 2>/dev/null || :; fi
    if [[ -n "$display_pid" ]]; then kill "$display_pid" 2>/dev/null || :; wait "$display_pid" 2>/dev/null || :; fi
    rm -f "$proof/output"
}
trap cleanup EXIT
coproc DISPLAY_SERVER { exec Xwayland -displayfd 1 -geometry 1000x800 -noreset -nolisten tcp -ac 2>"$proof/xwayland.log"; }
display_pid=$DISPLAY_SERVER_PID
IFS= read -r -t 15 display <&"${DISPLAY_SERVER[0]}" || { cat "$proof/xwayland.log"; exit 1; }
export DISPLAY=":$display" WAYLAND_DISPLAY=
pixels() {
    magick "$proof/$theme.png" -crop "$1" -colorspace gray -threshold 50% \
        -format %c histogram:info:- | awk -v color="$2" 'index($0,color) {gsub(":","",$1); print $1}'
}
for theme in light dark; do
    mkfifo "$proof/output"
    theme_env=(-u LIORA_SMOKE_DARK)
    color='#000000'
    if [[ "$theme" == dark ]]; then theme_env=(LIORA_SMOKE_DARK=1); color='#FFFFFF'; fi
    env "${theme_env[@]}" LIORA_NATIVE_SMOKE=1 LIORA_MARKDOWN_PREVIEW=1 \
        target/debug/examples/markdown_editing_smoke >"$proof/output" 2>"$proof/$theme.log" &
    app=$!
    exec {output}<"$proof/output"
    ready=false
    while IFS= read -r -t 15 line <&"$output"; do
        printf '%s\n' "$line" >>"$proof/$theme.log"
        if [[ "$line" == 'READY: native Markdown editing smoke' ]]; then ready=true; break; fi
    done
    "$ready" || { cat "$proof/$theme.log"; exit 1; }
    visible=false
    # Wait for actual presentation, not just the app's draw request.
    for attempt in {1..40}; do
        if timeout 2 env DISPLAY="$DISPLAY" import -window 'Liora Markdown Visibility Smoke' "$proof/$theme.png" 2>>"$proof/$theme.log"; then
            text=$(pixels 900x100+15+40 "$color")
            math=$(pixels 60x75+15+155 "$color")
            labels=$(pixels 70x25+40+275 "$color")
            edge=$(pixels 75x20+120+275 "$color")
            if [[ ${text:-0} -ge 500 && ${text:-0} -le 10000 && ${math:-0} -ge 100 && ${math:-0} -le 1500 && ${labels:-0} -ge 60 && ${labels:-0} -le 1000 && ${edge:-0} -ge 60 && ${edge:-0} -le 1000 ]]; then visible=true; break; fi
        fi
        sleep 0.1
    done
    "$visible" || { echo "FAIL: $theme text=${text:-0} math=${math:-0} labels=${labels:-0} edge=${edge:-0}; $proof"; exit 1; }
    wait "$app"
    app=
    exec {output}<&-
    rm "$proof/output"
    echo "PASS: $theme text=$text math=$math labels=$labels edge=$edge"
done
echo "Native visibility screenshots and logs: $proof"

#!/usr/bin/env python3
"""Drive a prebuilt native GPUI example through isolated fcitx5/XIM and XTest.

Build separately: cargo build -p liora-gallery --example markdown_real_ime_smoke --offline
Run: python3 scripts/markdown_real_ime_smoke.py [path/to/example]
Requires a working host Wayland compositor for a new, private rootful Xwayland.
No host fcitx configuration, clipboard, or desktop input is read or modified.
"""
import ctypes as C
import os
from pathlib import Path
import queue
import select
import shutil
import signal
import subprocess
import sys
import tempfile
import threading
import time

TITLE = b"Liora Real Markdown IME Smoke"
PASS = "PASS: real XIM paragraph/table preedit, 你好 commit, Markdown preservation and undo"


class X11:
    def __init__(self, display):
        self.lib = C.CDLL("libX11.so.6")
        self.xtest = C.CDLL("libXtst.so.6")
        signatures = {
            "XOpenDisplay": ([C.c_char_p], C.c_void_p),
            "XDefaultRootWindow": ([C.c_void_p], C.c_ulong),
            "XInternAtom": ([C.c_void_p, C.c_char_p, C.c_int], C.c_ulong),
            "XGetSelectionOwner": ([C.c_void_p, C.c_ulong], C.c_ulong),
            "XQueryTree": ([C.c_void_p, C.c_ulong, C.POINTER(C.c_ulong),
                            C.POINTER(C.c_ulong), C.POINTER(C.POINTER(C.c_ulong)),
                            C.POINTER(C.c_uint)], C.c_int),
            "XFetchName": ([C.c_void_p, C.c_ulong, C.POINTER(C.c_void_p)], C.c_int),
            "XFree": ([C.c_void_p], C.c_int),
            "XSetInputFocus": ([C.c_void_p, C.c_ulong, C.c_int, C.c_ulong], C.c_int),
            "XRaiseWindow": ([C.c_void_p, C.c_ulong], C.c_int),
            "XStringToKeysym": ([C.c_char_p], C.c_ulong),
            "XKeysymToKeycode": ([C.c_void_p, C.c_ulong], C.c_ubyte),
            "XSync": ([C.c_void_p, C.c_int], C.c_int),
            "XCloseDisplay": ([C.c_void_p], C.c_int),
        }
        for name, (args, result) in signatures.items():
            fn = getattr(self.lib, name)
            fn.argtypes, fn.restype = args, result
        self.xtest.XTestFakeKeyEvent.argtypes = [C.c_void_p, C.c_uint, C.c_int, C.c_ulong]
        self.xtest.XTestFakeKeyEvent.restype = C.c_int
        self.display = self.lib.XOpenDisplay(display.encode())
        if not self.display:
            raise RuntimeError("cannot connect to private Xwayland")
        self.root = self.lib.XDefaultRootWindow(self.display)

    def find_window(self, window=None):
        window = self.root if window is None else window
        name = C.c_void_p()
        if self.lib.XFetchName(self.display, window, C.byref(name)) and name.value:
            try:
                if C.string_at(name) == TITLE:
                    return window
            finally:
                self.lib.XFree(name)
        root, parent = C.c_ulong(), C.c_ulong()
        children, count = C.POINTER(C.c_ulong)(), C.c_uint()
        if self.lib.XQueryTree(self.display, window, C.byref(root), C.byref(parent),
                              C.byref(children), C.byref(count)):
            try:
                for i in range(count.value):
                    found = self.find_window(children[i])
                    if found:
                        return found
            finally:
                if children:
                    self.lib.XFree(children)
        return None

    def focus(self, window):
        self.lib.XSetInputFocus(self.display, self.root, 2, 0)
        self.lib.XSync(self.display, 0)
        self.lib.XRaiseWindow(self.display, window)
        self.lib.XSetInputFocus(self.display, window, 2, 0)
        self.lib.XSync(self.display, 0)

    def key(self, name):
        code = self.lib.XKeysymToKeycode(self.display, self.lib.XStringToKeysym(name.encode()))
        if not code:
            raise RuntimeError(f"no native keycode for {name}")
        for pressed in (1, 0):
            if not self.xtest.XTestFakeKeyEvent(self.display, code, pressed, 0):
                raise RuntimeError("XTest rejected key event")
            self.lib.XSync(self.display, 0)
            time.sleep(0.03)


def until(description, predicate, seconds=10):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        value = predicate()
        if value:
            return value
        time.sleep(0.05)
    raise RuntimeError(f"timed out: {description}")


def session(directory, binary):
    # This entry point must only run inside the private bus created below.
    if not os.environ.get("LIORA_REAL_IME_PRIVATE_BUS"):
        raise RuntimeError("private D-Bus session required")
    directory = Path(directory)
    host_env = os.environ.copy()
    host_env["XDG_RUNTIME_DIR"] = host_env.pop("LIORA_REAL_IME_HOST_RUNTIME")
    env = host_env.copy()
    for key in ("DISPLAY", "WAYLAND_DISPLAY", "WAYLAND_SOCKET", "XAUTHORITY",
                "FCITX_CONFIG_HOME", "FCITX_DATA_HOME", "IBUS_ADDRESS"):
        env.pop(key, None)
    for key, name in (("HOME", "home"), ("XDG_CONFIG_HOME", "config"),
                      ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"),
                      ("XDG_STATE_HOME", "state"), ("XDG_RUNTIME_DIR", "runtime")):
        path = directory / name
        path.mkdir(mode=0o700, exist_ok=True)
        env[key] = str(path)
    env.update(XMODIFIERS="@im=fcitx", GTK_IM_MODULE="xim", QT_IM_MODULE="xim",
               LC_ALL="C.UTF-8", LIORA_REAL_IME_ISOLATED="1",
               XDG_CONFIG_DIRS="/etc/xdg", XDG_DATA_DIRS="/usr/local/share:/usr/share")
    config = directory / "config/fcitx5"
    (config / "conf").mkdir(parents=True)
    (config / "profile").write_text(
        "[Groups/0]\nName=Default\nDefault Layout=us\nDefaultIM=pinyin\n\n"
        "[Groups/0/Items/0]\nName=keyboard-us\nLayout=\n\n"
        "[Groups/0/Items/1]\nName=pinyin\nLayout=\n\n[GroupOrder]\n0=Default\n")
    (config / "config").write_text(
        "[Behavior]\nActiveByDefault=True\nPreeditEnabledByDefault=True\n")
    # GPUI requests PREEDIT_CALLBACKS; fcitx disables on-the-spot XIM by default.
    (config / "conf/xim.conf").write_text("UseOnTheSpot=True\n")
    processes, logs = [], []
    x11 = None
    success = False
    try:
        def launch(command, name, process_env, **kwargs):
            log = (directory / f"{name}.log").open("w+")
            logs.append((name, log))
            process = subprocess.Popen(command, env=process_env, stderr=log,
                                       stdout=kwargs.pop("stdout", log), **kwargs)
            processes.append(process)
            return process

        # Keep only Xwayland's compositor connection on the host runtime directory.
        xwayland = launch(["Xwayland", "-displayfd", "1", "-ac", "-nolisten", "tcp", "-noreset"],
                          "xwayland", host_env, stdout=subprocess.PIPE)
        if not select.select([xwayland.stdout], [], [], 10)[0]:
            raise RuntimeError("Xwayland did not publish a private display")
        number = xwayland.stdout.readline().decode().strip()
        if not number.isdecimal():
            raise RuntimeError(f"invalid private Xwayland display: {number!r}")
        env["DISPLAY"] = f":{number}"
        print(f"Isolated Xwayland {env['DISPLAY']}; private D-Bus and temporary HOME/XDG", flush=True)
        x11 = X11(env["DISPLAY"])
        fcitx = launch(["fcitx5", "--keep", "--disable=wayland,waylandim,clipboard"], "fcitx", env)
        atom = x11.lib.XInternAtom(x11.display, b"@server=fcitx", 0)
        until("fcitx XIM selection", lambda: fcitx.poll() is None and
              x11.lib.XGetSelectionOwner(x11.display, atom))
        app = launch([binary], "app", env, stdout=subprocess.PIPE, text=True, bufsize=1)
        lines = queue.Queue()

        def read_output():
            for line in app.stdout:
                line = line.rstrip("\n")
                if line != PASS:
                    print(line, flush=True)
                lines.put(line)
            lines.put(None)

        threading.Thread(target=read_output, daemon=True).start()

        def expect(expected):
            deadline = time.monotonic() + 20
            while time.monotonic() < deadline:
                try:
                    line = lines.get(timeout=max(0.01, deadline - time.monotonic()))
                except queue.Empty:
                    break
                if line is None:
                    raise RuntimeError(f"app exited before {expected!r} (status {app.poll()})")
                if line == expected:
                    return
            raise RuntimeError(f"missing handshake: {expected}")

        for name in ("paragraph", "table"):
            expect(f"READY: {name}")
            window = until("named GPUI window", x11.find_window)
            x11.focus(window)
            # Remote only selects/activates the engine on our new D-Bus; all text is XTest.
            for args in (("-s", "pinyin"), ("-o",)):
                subprocess.run(["fcitx5-remote", *args], env=env, check=True, timeout=5)
            # GPUI's pinned XIM client does not send XIM_SET_IC_FOCUS; fcitx
            # discovers this context on its first forwarded native key event.
            time.sleep(0.3)
            for key in "nihao":
                x11.key(key)
            expect(f"PREEDIT: {name}; canonical unchanged")
            # Hold the actual composition so the native assertions observe it repeatedly.
            time.sleep(0.3)
            x11.key("space")
            expect(f"COMMIT: {name}; 你好; Markdown preserved")
            expect(f"UNDO: {name}; original restored")
        expect(PASS)
        if app.wait(timeout=5) != 0:
            raise RuntimeError("native example failed after PASS handshake")
        success = True
    finally:
        if x11 is not None:
            x11.lib.XCloseDisplay(x11.display)
        for process in reversed(processes):
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait(timeout=3)
        for name, log in logs:
            if not success:
                log.seek(0)
                print(f"--- {name} stderr/log ---\n{log.read()}", file=sys.stderr)
            log.close()


def main():
    if len(sys.argv) == 4 and sys.argv[1] == "--session":
        session(sys.argv[2], sys.argv[3])
        return 0
    if len(sys.argv) > 2:
        raise RuntimeError("usage: markdown_real_ime_smoke.py [prebuilt-example]")
    root = Path(__file__).resolve().parent.parent
    binary = Path(sys.argv[1]) if len(sys.argv) == 2 else root / "target/debug/examples/markdown_real_ime_smoke"
    binary = binary.resolve()
    if not binary.is_file() or not os.access(binary, os.X_OK):
        raise RuntimeError(f"build the example first; executable not found: {binary}")
    for command in ("dbus-run-session", "Xwayland", "fcitx5", "fcitx5-remote"):
        if not shutil.which(command):
            raise RuntimeError(f"required command missing: {command}")
    if not os.environ.get("WAYLAND_DISPLAY") or not os.environ.get("XDG_RUNTIME_DIR"):
        raise RuntimeError("a host Wayland compositor is required for isolated Xwayland")
    with tempfile.TemporaryDirectory(prefix="liora-real-ime-") as directory:
        env = os.environ.copy()
        # Never permit a remote command to reach the desktop's bus or IME socket.
        for key in ("DBUS_SESSION_BUS_ADDRESS", "DBUS_SESSION_BUS_PID", "DBUS_SESSION_BUS_WINDOWID"):
            env.pop(key, None)
        env["LIORA_REAL_IME_PRIVATE_BUS"] = "1"
        env["LIORA_REAL_IME_HOST_RUNTIME"] = env["XDG_RUNTIME_DIR"]
        for key, name in (("HOME", "home"), ("XDG_CONFIG_HOME", "config"),
                          ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"),
                          ("XDG_STATE_HOME", "state"), ("XDG_RUNTIME_DIR", "runtime")):
            path = Path(directory) / name
            path.mkdir(mode=0o700)
            env[key] = str(path)
        env.update(XDG_CONFIG_DIRS="/etc/xdg", XDG_DATA_DIRS="/usr/local/share:/usr/share")
        # Adopt grandchildren so even Xlib's fatal exit cannot strand the bus,
        # Xwayland, fcitx or D-Bus-activated services after their parent exits.
        libc = C.CDLL(None, use_errno=True)
        if libc.prctl(36, 1, 0, 0, 0) != 0:  # PR_SET_CHILD_SUBREAPER
            raise OSError(C.get_errno(), "cannot supervise IME child processes")
        process = subprocess.Popen(["dbus-run-session", "--", sys.executable,
                                    str(Path(__file__).resolve()), "--session", directory, str(binary)],
                                   env=env, start_new_session=True)
        def interrupted(signum, frame):
            raise RuntimeError(f"interrupted by signal {signum}")
        handlers = {sig: signal.signal(sig, interrupted) for sig in (signal.SIGTERM, signal.SIGINT)}
        try:
            result = process.wait(timeout=90)
        finally:
            for sig, handler in handlers.items():
                signal.signal(sig, handler)
            for sig in (signal.SIGTERM, signal.SIGKILL):
                try:
                    os.killpg(process.pid, sig)
                except ProcessLookupError:
                    pass
                if sig == signal.SIGTERM:
                    time.sleep(0.3)
            process.wait(timeout=5)
            deadline = time.monotonic() + 5
            while True:
                try:
                    pid, _ = os.waitpid(-process.pid, os.WNOHANG)
                except ChildProcessError:
                    break
                if pid == 0:
                    if time.monotonic() >= deadline:
                        raise RuntimeError("isolated IME process group did not reap")
                    time.sleep(0.05)
        if result == 0:
            print(PASS, flush=True)
        return result


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, OSError, subprocess.SubprocessError) as error:
        print(f"FAIL: real IME smoke: {error}", file=sys.stderr)
        sys.exit(1)

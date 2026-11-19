"""Run: python3 scripts/system_appearance_native.py OUTPUT_DIR NATIVE_BINARY system

Requires dbus-daemon, Python dbus-next, KWin Wayland, Xwayland and software Vulkan.
The public entry always creates a fresh bus; OUTPUT_DIR must not already exist.
For the paired fixed-light negative control only, set LIORA_SUPPRESS_SEQ1=1.
"""
import asyncio, hashlib, json, os, secrets, signal, struct, subprocess, sys, time
from pathlib import Path
from dbus_next import Variant, DBusError, PropertyAccess, Message, MessageType
from dbus_next.aio import MessageBus
from dbus_next.service import ServiceInterface, method, signal as dbus_signal, dbus_property


def sha256(path):
    with Path(path).open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()
async def read_line(stream, limit=256):
    os.set_blocking(stream.fileno(), False)
    data = bytearray()
    async with asyncio.timeout(12):
        while b'\n' not in data:
            chunk = os.read(stream.fileno(), limit - len(data)) if select_ready(stream) else None
            if chunk == b'':
                raise RuntimeError('readiness pipe closed')
            if chunk:
                data.extend(chunk)
                assert len(data) < limit, 'oversized readiness line'
            await asyncio.sleep(.025)
    return data.decode().strip()

def select_ready(stream):
    import select
    return bool(select.select([stream], [], [], 0)[0])

async def main():
    assert len(sys.argv) == 4, 'expected OUTPUT_DIR NATIVE_BINARY CASE'
    root = Path(sys.argv[1]).resolve(); root.mkdir(mode=0o700)
    owned=[]; logs=[]
    try:
        async with asyncio.timeout(90):
            log=(root/'dbus.log').open('w'); logs.append(log)
            daemon=subprocess.Popen(['dbus-daemon', '--session', '--nofork', '--print-address=1'],
                stdout=subprocess.PIPE, stderr=log, start_new_session=True)
            owned.append(daemon)
            address=await read_line(daemon.stdout)
            assert address.startswith('unix:'), 'private daemon did not publish a Unix address'
            return await run(root, address, owned, logs)
    finally:
        failures=[]
        # Leaders may have exited while their process group still has descendants.
        for sig in (signal.SIGTERM, signal.SIGKILL):
            for process in reversed(owned):
                try: os.killpg(process.pid, sig)
                except ProcessLookupError: pass
                except OSError as error: failures.append(str(error))
            if sig == signal.SIGTERM:
                await asyncio.sleep(.25)
        for process in reversed(owned):
            try: process.wait(timeout=2)
            except (OSError, subprocess.TimeoutExpired) as error: failures.append(str(error))
            if process.stdout: process.stdout.close()
        for log in logs: log.close()
        print('Owned portal/display/application process groups stopped', flush=True)
        assert not failures, failures

class Settings(ServiceInterface):
    def __init__(self):
        super().__init__('org.freedesktop.portal.Settings')
        self.scheme = 2
    @dbus_property(access=PropertyAccess.READ)
    def version(self) -> 'u':
        return 1
    @method()
    def Read(self, namespace: 's', key: 's') -> 'v':
        print('READ', namespace, key, flush=True)
        values = {('org.freedesktop.appearance', 'color-scheme'): Variant('u', self.scheme),
                  ('org.gnome.desktop.interface', 'cursor-theme'): Variant('s', 'Adwaita'),
                  ('org.gnome.desktop.interface', 'cursor-size'): Variant('i', 24),
                  ('org.gnome.desktop.wm.preferences', 'button-layout'): Variant('s', ':minimize,maximize,close')}
        if (namespace, key) not in values:
            raise DBusError('org.freedesktop.portal.Error.NotFound', 'Unknown setting')
        return values[namespace, key]
    @dbus_signal()
    def SettingChanged(self, namespace: 's', key: 's', value: 'v') -> 'ssv':
        return [namespace, key, value]

async def run(root, address, owned, logs):
    binary = Path(sys.argv[2]).resolve(strict=True); case = sys.argv[3]
    assert case in ('system', 'fixed-dark', 'fixed-light'), 'unknown case'
    suppress = os.environ.get('LIORA_SUPPRESS_SEQ1') == '1'
    assert not suppress or case == 'fixed-light', 'negative control requires fixed-light'
    env = os.environ.copy()
    env['DBUS_SESSION_BUS_ADDRESS'] = address
    for key in ('DISPLAY', 'WAYLAND_DISPLAY', 'WAYLAND_SOCKET', 'XAUTHORITY', 'AT_SPI_BUS_ADDRESS', 'DBUS_STARTER_ADDRESS', 'DBUS_STARTER_BUS_TYPE'):
        env.pop(key, None)
    for key, name in {'HOME':'home','XDG_RUNTIME_DIR':'runtime','XDG_CONFIG_HOME':'config','XDG_DATA_HOME':'data','XDG_CACHE_HOME':'cache','XDG_STATE_HOME':'state'}.items():
        path=root/name; path.mkdir(mode=0o700); env[key]=str(path)
    gtk=root/'home/.config/gtk-4.0'; gtk.mkdir(parents=True)
    (gtk/'settings.ini').write_text('[Settings]\ngtk-application-prefer-dark-theme=false\n')
    env.update(QT_QPA_PLATFORM='offscreen', KWIN_COMPOSE='Q', LIBGL_ALWAYS_SOFTWARE='1', GSETTINGS_BACKEND='memory', GTK_THEME='Adwaita', NO_AT_BRIDGE='1')
    if case == 'fixed-light':
        env['GTK_THEME'] = 'Adwaita-dark'
    # Prevent subprocesses from consulting the host gsettings executable.
    bin_dir=root/'private-bin'; bin_dir.mkdir()
    deny=bin_dir/'gsettings'; deny.write_text('#!/bin/sh\necho forbidden-gsettings >&2\nexit 99\n'); deny.chmod(0o700)
    env['PATH']=str(bin_dir)+':'+env['PATH']
    bus = await MessageBus(bus_address=address).connect()
    service = Settings(); bus.export('/org/freedesktop/portal/desktop', service)
    await bus.request_name('org.freedesktop.portal.Desktop')
    monitor = await MessageBus(bus_address=address).connect()
    reads = {}; subscriptions = {}; pending = {}
    rule = ("type='signal',sender='org.freedesktop.portal.Desktop',"
            "interface='org.freedesktop.portal.Settings',member='SettingChanged',"
            "path='/org/freedesktop/portal/desktop',"
            "arg0='org.freedesktop.appearance',arg1='color-scheme'")
    def observed(message):
        if message.message_type == MessageType.METHOD_CALL:
            kind = None
            if (message.destination in ('org.freedesktop.portal.Desktop', bus.unique_name)
                    and message.path == '/org/freedesktop/portal/desktop'
                    and message.interface == 'org.freedesktop.portal.Settings'
                    and message.member == 'Read' and message.signature == 'ss'
                    and message.body == ['org.freedesktop.appearance', 'color-scheme']):
                kind = 'Read'
            if (message.destination == 'org.freedesktop.DBus'
                    and message.path == '/org/freedesktop/DBus'
                    and message.interface == 'org.freedesktop.DBus'
                    and message.member == 'AddMatch' and message.signature == 's'
                    and message.body == [rule] and message.sender in reads):
                kind = 'AddMatch'
            if kind:
                pending[(message.sender, message.serial)] = kind
        elif message.message_type == MessageType.METHOD_RETURN:
            key = (message.destination, message.reply_serial)
            kind = pending.get(key)
            read_ok = (kind == 'Read' and message.sender == bus.unique_name
                and message.signature == 'v' and len(message.body) == 1
                and message.body[0] == Variant('u', 2))
            match_ok = (kind == 'AddMatch' and message.sender == 'org.freedesktop.DBus'
                and message.signature == '' and message.body == [])
            if read_ok or match_ok:
                record = {'unique_name': key[0], 'call_serial': key[1],
                    'reply_sender': message.sender, 'reply_serial': message.serial,
                    'reply_to': message.reply_serial, 'kind': kind}
                (reads if read_ok else subscriptions)[key[0]] = record
                print('HANDSHAKE', json.dumps(record), flush=True)
                del pending[key]
        # A monitor observes calls addressed to other clients; never auto-reply.
        return message.message_type == MessageType.METHOD_CALL
    monitor.add_message_handler(observed)
    reply = await monitor.call(Message(
        destination='org.freedesktop.DBus', path='/org/freedesktop/DBus',
        interface='org.freedesktop.DBus.Monitoring', member='BecomeMonitor',
        signature='asu', body=[[], 0]))
    assert reply.message_type == MessageType.METHOD_RETURN, reply.body
    app = None
    try:
        log=(root/'kwin.log').open('w'); logs.append(log)
        p=subprocess.Popen(['/usr/bin/kwin_wayland','--virtual','--width','1024','--height','768','--socket','a','--no-lockscreen','--no-global-shortcuts','--no-kactivities'],env=env,stdout=log,stderr=log,start_new_session=True); owned.append(p)
        deadline=time.monotonic()+15; socket=root/'runtime/a'
        while not socket.is_socket():
            assert p.poll() is None and time.monotonic()<deadline, 'compositor readiness failed'
            await asyncio.sleep(.05)
        env['WAYLAND_DISPLAY']=str(socket)
        record=lambda b:struct.pack('>H',len(b))+b
        auth=root/'xauth'; auth.write_bytes(b'\xff\xff'+b'\x00\x00'*2+record(b'MIT-MAGIC-COOKIE-1')+record(secrets.token_bytes(16))); auth.chmod(0o600)
        env['XAUTHORITY']=str(auth)
        xlog=(root/'xwayland.log').open('w'); logs.append(xlog)
        x=subprocess.Popen(['/usr/bin/Xwayland','-displayfd','1','-auth',str(auth),'-shm','-noreset','-nolisten','tcp','-geometry','1024x768'],env=env,stdout=subprocess.PIPE,stderr=xlog,start_new_session=True); owned.append(x)
        number=await read_line(x.stdout); assert number.isdecimal()
        env['DISPLAY']=':'+number; env.pop('WAYLAND_DISPLAY'); env['XMODIFIERS']='@im=none'
        app_log=(root/'app.log').open('w'); logs.append(app_log)
        app=subprocess.Popen([str(binary),str(root),case],env=env,stdout=app_log,stderr=app_log,start_new_session=True); owned.append(app)
        await asyncio.sleep(.05)
        actual=Path(f'/proc/{app.pid}/exe').resolve(strict=True)
        assert actual == binary, (actual, binary)
        digest=sha256(Path(f'/proc/{app.pid}/exe'))
        (root/'identity.json').write_text(json.dumps({'pid':app.pid,'exe':str(actual),'sha256':digest,'argv':[str(binary),str(root),case],'gtk_theme':env['GTK_THEME'],'display':env['DISPLAY'],'bus':env['DBUS_SESSION_BUS_ADDRESS']},indent=2))
        last=None; verified=set()
        while app.poll() is None:
            for name in (reads.keys() & subscriptions.keys()) - verified:
                pid_reply = await bus.call(Message(destination='org.freedesktop.DBus',
                    path='/org/freedesktop/DBus', interface='org.freedesktop.DBus',
                    member='GetConnectionUnixProcessID', signature='s', body=[name]))
                assert (pid_reply.message_type == MessageType.METHOD_RETURN
                    and pid_reply.sender == 'org.freedesktop.DBus'
                    and pid_reply.signature == 'u'), pid_reply.body
                if pid_reply.body == [app.pid]:
                    record = {'pid': app.pid, 'pid_reply_sender': pid_reply.sender,
                        'pid_reply_serial': pid_reply.serial, 'pid_reply_to': pid_reply.reply_serial,
                        'Read': reads[name], 'AddMatch': subscriptions[name], 'match_rule': rule}
                    (root/'subscription-ready').write_text(json.dumps(record))
                    print('READY_PID', json.dumps(record), flush=True)
                verified.add(name)
            request=root/'request'
            if request.exists():
                seq, scheme = map(int, request.read_text().split())
                if seq != last:
                    assert scheme in (1,2)
                    service.scheme=scheme
                    if not (suppress and seq == 1):
                        service.SettingChanged('org.freedesktop.appearance','color-scheme',Variant('u',scheme))
                        print('EMIT',seq,scheme,flush=True)
                    else:
                        print('SUPPRESS',seq,scheme,'scheme/ack unchanged',flush=True)
                    (root/'ack').write_text(str(seq)); last=seq
            await asyncio.sleep(.025)
        (root/'result.json').write_text(json.dumps({'exit_code':app.returncode,'last_portal_sequence':last}))
        print((root/'app.log').read_text(),flush=True)
        if suppress:
            text=(root/'app.log').read_text()
            assert app.returncode != 0 and last == 1, 'negative control did not stop at seq1'
            assert 'portal delivery timeout seq=1' in text, 'negative control failed for a different reason'
            assert 'PASS native appearance scenario' not in text, 'negative control reached sentinel'
            print('PASS negative control: seq1 timed out without SettingChanged',flush=True)
        if app.returncode == 0:
            assert f'PASS native appearance scenario {case}' in (root/'app.log').read_text(), 'application exited before scenario completion'
            assert last == 3, 'no-window/reverse portal stages did not finish'
        return app.returncode
    finally:
        bus.disconnect()
        monitor.disconnect()

sys.exit(asyncio.run(main()))

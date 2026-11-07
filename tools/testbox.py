#!/usr/bin/env python3
"""A shared test computer on a separate test machine: a Trindle image
running in QEMU, one at a time, thrown away when it stops. Nothing it does
reaches that machine's own system or screen.

    testbox.py start --who NAME [--iso FILE] [--minutes N] [--wait]
    testbox.py install FILE.deb ...     copy in and install with apt
    testbox.py put LOCAL REMOTE         copy a file in
    testbox.py get REMOTE LOCAL         copy a file out
    testbox.py run 'command'            run as root; prints output, exits with its code
    testbox.py user 'command'           run as the person, inside their desktop
    testbox.py spawn 'command'          start a program as the person and leave it running
    testbox.py shot FILE.png            a picture of the screen (1280 x 800)
    testbox.py click X Y [left|right|middle] | scroll X Y STEPS | type TEXT | key NAME [NAME...]
    testbox.py stop | status

The lock is /var/tmp/trindle-testbox/lock. Anything else that needs the
laptop to itself, such as a hidden Xvfb screen, takes the same lock:
    flock /var/tmp/trindle-testbox/lock sh my-test.sh
"""
import base64, fcntl, glob, json, os, shutil, socket, subprocess, sys, time

HOME = "/var/tmp/trindle-testbox"
RUN = HOME + "/run"
LOCK = HOME + "/lock"
QMP = RUN + "/qmp.sock"
AGENT = RUN + "/agent.sock"
WHO = RUN + "/who"
OVMF = "/usr/share/OVMF/OVMF_CODE_4M.fd"
VARS = "/usr/share/OVMF/OVMF_VARS_4M.fd"
WIDTH, HEIGHT = 1280, 800
BOOT_SECONDS = 240

GUEST_AGENT = r'''
import base64, glob, json, os, pwd, subprocess
port = open("/dev/virtio-ports/testbox", "r+b", buffering=0)
def person():
    p = pwd.getpwnam("trindle")
    rt = "/run/user/%d" % p.pw_uid
    ways = sorted(w for w in glob.glob(rt + "/wayland-*") if not w.endswith(".lock"))
    env = {"HOME": p.pw_dir, "USER": "trindle", "LOGNAME": "trindle", "PATH": "/usr/local/bin:/usr/bin:/bin",
           "XDG_RUNTIME_DIR": rt, "DBUS_SESSION_BUS_ADDRESS": "unix:path=%s/bus" % rt, "LANG": "en_GB.UTF-8"}
    if ways:
        env["WAYLAND_DISPLAY"] = os.path.basename(ways[0])
    return ["runuser", "-u", "trindle", "--", "env", "-i"] + ["%s=%s" % kv for kv in env.items()]
def handle(r):
    op = r["op"]
    if op == "hello":
        return {"ok": True}
    if op == "put":
        os.makedirs(os.path.dirname(r["path"]) or "/", exist_ok=True)
        with open(r["path"], "wb") as f:
            f.write(base64.b64decode(r["data"]))
        os.chmod(r["path"], 0o644)
        return {"ok": True}
    if op == "get":
        with open(r["path"], "rb") as f:
            return {"ok": True, "data": base64.b64encode(f.read()).decode()}
    pre = person() if op in ("user", "spawn") else []
    if op == "spawn":
        log = open("/tmp/testbox-spawn.log", "ab")
        subprocess.Popen(pre + ["sh", "-c", r["cmd"]], stdin=subprocess.DEVNULL, stdout=log, stderr=log, start_new_session=True)
        return {"ok": True, "rc": 0, "out": ""}
    try:
        p = subprocess.run(pre + ["sh", "-c", r["cmd"]], stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                           stderr=subprocess.STDOUT, timeout=r.get("timeout", 600))
        return {"ok": True, "rc": p.returncode, "out": p.stdout.decode(errors="replace")}
    except subprocess.TimeoutExpired as e:
        return {"ok": True, "rc": 124, "out": (e.stdout or b"").decode(errors="replace") + "\n(timed out)\n"}
buf = b""
while True:
    chunk = port.read(65536)
    if not chunk:
        continue
    buf += chunk
    while b"\n" in buf:
        line, buf = buf.split(b"\n", 1)
        try:
            out = handle(json.loads(line))
        except Exception as e:
            out = {"ok": False, "error": str(e)}
        port.write(json.dumps(out).encode() + b"\n")
'''

HOOK = """#!/bin/sh
cat > /usr/local/sbin/testbox-agent <<'AGENT'
%s
AGENT
cat > /etc/systemd/system/testbox-agent.service <<'UNIT'
[Unit]
Description=Test computer agent
[Service]
ExecStart=/usr/bin/python3 /usr/local/sbin/testbox-agent
Restart=always
UNIT
systemctl daemon-reload
systemctl --no-block start testbox-agent.service
""" % GUEST_AGENT

def die(msg, code=1):
    print(msg, file=sys.stderr)
    sys.exit(code)

def running():
    try:
        fd = os.open(LOCK, os.O_RDWR | os.O_CREAT, 0o666)
    except OSError:
        return True
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        fcntl.flock(fd, fcntl.LOCK_UN)
        return False
    except BlockingIOError:
        return True
    finally:
        os.close(fd)

def who():
    try:
        return open(WHO).read().strip()
    except OSError:
        return "another test (not the test computer)"

def take_out(iso, names, into):
    """Copies the named files out of a disc image into a folder, with
    libarchive, the library Folders reads disc images with, so no archive
    program has to be installed."""
    import ctypes
    la, ptr = ctypes.CDLL("libarchive.so.13"), ctypes.c_void_p
    la.archive_read_new.restype = ptr
    la.archive_entry_pathname.restype = ctypes.c_char_p
    la.archive_entry_pathname.argtypes = [ptr]
    la.archive_read_data.restype = ctypes.c_ssize_t
    la.archive_read_data.argtypes = [ptr, ctypes.c_void_p, ctypes.c_size_t]
    la.archive_read_open_filename.argtypes = [ptr, ctypes.c_char_p, ctypes.c_size_t]
    la.archive_read_next_header.argtypes = [ptr, ctypes.POINTER(ptr)]
    la.archive_read_support_format_iso9660.argtypes = [ptr]
    la.archive_read_free.argtypes = [ptr]
    a = ptr(la.archive_read_new())
    la.archive_read_support_format_iso9660(a)
    if la.archive_read_open_filename(a, iso.encode(), 1 << 16) != 0:
        sys.exit(iso + " is not a disc image that can be read")
    entry, buf, left = ptr(), ctypes.create_string_buffer(1 << 16), set(names)
    while left and la.archive_read_next_header(a, ctypes.byref(entry)) in (0, -20):
        name = (la.archive_entry_pathname(entry) or b"").decode(errors="replace").lstrip("./")
        if name not in left:
            continue
        os.makedirs(os.path.dirname(into + "/" + name), exist_ok=True)
        with open(into + "/" + name + ".part", "wb") as out:
            while True:
                n = la.archive_read_data(a, buf, len(buf))
                if n < 0:
                    sys.exit(name + " could not be read out of " + iso)
                if n == 0:
                    break
                out.write(buf.raw[:n])
        os.replace(into + "/" + name + ".part", into + "/" + name)
        left.discard(name)
    la.archive_read_free(a)
    if left:
        sys.exit("not in " + iso + ": " + ", ".join(sorted(left)))

def boot_files(iso):
    """The image's kernel and start-up files, taken out once per image."""
    cache = HOME + "/boot/" + os.path.basename(iso)
    if not os.path.exists(cache + "/live/initrd.img"):
        os.makedirs(cache, exist_ok=True)
        take_out(iso, ["live/vmlinuz", "live/initrd.img"], cache)
    return cache + "/live/vmlinuz", cache + "/live/initrd.img"

def start(args):
    whoami, iso, minutes, wait = None, None, 60, False
    while args:
        a = args.pop(0)
        if a == "--who": whoami = args.pop(0)
        elif a == "--iso": iso = args.pop(0)
        elif a == "--minutes": minutes = int(args.pop(0))
        elif a == "--wait": wait = True
        else: die("unknown: " + a)
    if not whoami:
        die("say who is testing: --who Talk")
    iso = iso or (sorted(glob.glob("/var/tmp/trindle-iso/trindle-*-amd64.iso")) or [None])[-1]
    if not iso or not os.path.exists(iso):
        die("no image found; name one with --iso")
    os.makedirs(RUN, exist_ok=True)
    fd = os.open(LOCK, os.O_RDWR | os.O_CREAT, 0o666)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | (0 if wait else fcntl.LOCK_NB))
    except BlockingIOError:
        die("busy: " + who() + ". Try again later, or add --wait.", 3)
    kernel, initrd = boot_files(iso)
    with open(WHO, "w") as f:
        f.write("%s since %s, image %s\n" % (whoami, time.strftime("%H:%M"), os.path.basename(iso)))
    for s in (QMP, AGENT):
        if os.path.exists(s):
            os.unlink(s)
    vars_copy = RUN + "/vars.fd"
    with open(VARS, "rb") as a, open(vars_copy, "wb") as b:
        b.write(a.read())
    blank = RUN + "/disk.img"
    with open(blank, "wb") as f:
        f.truncate(32 << 30)
    share = RUN + "/share"
    os.makedirs(share, exist_ok=True)
    with open(share + "/hook.sh", "w") as f:
        f.write(HOOK)
    append = ("boot=live components quiet splash vt.global_cursor_default=0 username=trindle "
              "user-fullname=Trindle hostname=trindle systemd.mount-extra=/dev/vda1:/run/testbox:vfat:ro "
              "live-config.hooks=file:///run/testbox/hook.sh")
    cmd = ["timeout", "--kill-after=10", str(minutes * 60),
           "qemu-system-x86_64", "-enable-kvm", "-m", "4096", "-smp", "4", "-machine", "q35",
           "-drive", "if=pflash,format=raw,readonly=on,file=" + OVMF,
           "-drive", "if=pflash,format=raw,file=" + vars_copy,
           "-kernel", kernel, "-initrd", initrd, "-append", append,
           "-drive", "file=%s,media=cdrom,readonly=on" % iso,
           "-drive", "file=fat:ro:%s,format=raw,if=virtio,readonly=on" % share,
           "-drive", "file=%s,format=raw,if=virtio" % blank,
           "-netdev", "user,id=n", "-device", "virtio-net-pci,netdev=n",
           "-device", "qemu-xhci", "-device", "usb-tablet", "-vga", "virtio", "-display", "none",
           "-qmp", "unix:%s,server=on,wait=off" % QMP,
           "-device", "virtio-serial",
           "-chardev", "socket,id=tb,path=%s,server=on,wait=off" % AGENT,
           "-device", "virtserialport,chardev=tb,name=testbox"]
    log = open(RUN + "/qemu.log", "w")
    subprocess.Popen(cmd, stdin=subprocess.DEVNULL, stdout=log, stderr=log, pass_fds=(fd,), start_new_session=True)
    os.close(fd)
    end = time.time() + BOOT_SECONDS
    while time.time() < end:
        time.sleep(3)
        if not running():
            die("the test computer stopped while starting; see " + RUN + "/qemu.log")
        try:
            if agent({"op": "hello"}, timeout=3).get("ok"):
                while time.time() < end:
                    r = agent({"op": "run", "cmd": "ls /run/user/*/wayland-* 2>/dev/null | grep -v lock"}, timeout=10)
                    if r.get("out", "").strip():
                        agent({"op": "run", "cmd": "for i in $(seq 30); do (pkill -x trindle-live-we || pkill -x trindle-welcome) && break; sleep 1; done; true"}, timeout=40)
                        print("ready: %s. It stops by itself after %d minutes." % (os.path.basename(iso), minutes))
                        return
                    time.sleep(3)
        except OSError:
            pass
    die("the test computer did not answer in time; stop it and see " + RUN + "/qemu.log")

def agent(req, timeout=660):
    s = socket.socket(socket.AF_UNIX)
    s.settimeout(timeout)
    s.connect(AGENT)
    s.sendall(json.dumps(req).encode() + b"\n")
    buf = b""
    while not buf.endswith(b"\n"):
        chunk = s.recv(1 << 20)
        if not chunk:
            raise OSError("no answer")
        buf += chunk
    s.close()
    r = json.loads(buf)
    if not r.get("ok"):
        die("failed inside the test computer: " + r.get("error", "?"))
    return r

def qmp(*cmds):
    s = socket.socket(socket.AF_UNIX)
    s.connect(QMP)
    f = s.makefile("rw")
    f.readline()
    def c(x):
        f.write(json.dumps(x) + "\n"); f.flush()
        while True:
            r = json.loads(f.readline())
            if "event" not in r:
                return r
    c({"execute": "qmp_capabilities"})
    out = [c(x) for x in cmds]
    s.close()
    return out

def pointer(x, y):
    return {"execute": "input-send-event", "arguments": {"events": [
        {"type": "abs", "data": {"axis": "x", "value": int(float(x) * 32767 / WIDTH)}},
        {"type": "abs", "data": {"axis": "y", "value": int(float(y) * 32767 / HEIGHT)}}]}}

def button(down, which="left"):
    return {"execute": "input-send-event", "arguments": {"events": [
        {"type": "btn", "data": {"down": down, "button": which}}]}}

KEYS = {" ": "spc", "-": "minus", ".": "dot", "/": "slash", ",": "comma", "=": "equal", ";": "semicolon",
        "'": "apostrophe", "\n": "ret", "\t": "tab", "\\": "backslash", "[": "bracket_left", "]": "bracket_right"}
SHIFTED = {"!": "1", "@": "apostrophe", "#": "3", "$": "4", "%": "5", "^": "6", "&": "7", "*": "8", "(": "9",
           ")": "0", "_": "minus", "+": "equal", ":": "semicolon", "?": "slash", "<": "comma", ">": "dot",
           "\"": "2", "{": "bracket_left", "}": "bracket_right", "|": "backslash"}

def keys(names):
    return {"execute": "send-key", "arguments": {"keys": [{"type": "qcode", "data": k} for k in names]}}

def main():
    a = sys.argv[1:]
    if not a:
        print(__doc__)
        return
    op = a.pop(0)
    if op == "start":
        return start(a)
    if op == "status":
        print(who() if running() else "free")
        return
    if not running() or not os.path.exists(AGENT):
        die("the test computer is not running; start it first", 2)
    if op == "stop":
        qmp({"execute": "quit"})
        while running():
            time.sleep(0.5)
        if os.path.exists(RUN + "/disk.img"):
            os.unlink(RUN + "/disk.img")
        print("stopped; nothing kept")
    elif op in ("run", "user"):
        r = agent({"op": op, "cmd": a[0]})
        sys.stdout.write(r["out"])
        sys.exit(r["rc"])
    elif op == "spawn":
        agent({"op": "spawn", "cmd": a[0]})
    elif op == "put":
        agent({"op": "put", "path": a[1], "data": base64.b64encode(open(a[0], "rb").read()).decode()})
    elif op == "get":
        open(a[1], "wb").write(base64.b64decode(agent({"op": "get", "path": a[0]})["data"]))
    elif op == "install":
        names = []
        for deb in a:
            if not os.path.isfile(deb):
                die("no such file: " + deb)
            names.append("/tmp/testbox-debs/" + os.path.basename(deb))
            agent({"op": "put", "path": names[-1], "data": base64.b64encode(open(deb, "rb").read()).decode()})
        r = agent({"op": "run", "cmd": "DEBIAN_FRONTEND=noninteractive apt-get install -y %s 2>&1 | tr '\\r' '\\n' | grep -v '^(Reading' | tail -5" % " ".join(names)})
        sys.stdout.write(r["out"])
    elif op == "shot":
        tmp = RUN + "/shot.png"
        r = qmp({"execute": "screendump", "arguments": {"filename": tmp, "format": "png"}})[0]
        if "error" in r:
            die(r["error"]["desc"])
        shutil.move(tmp, a[0])
    elif op == "click":
        qmp(pointer(float(a[0]) + 1, a[1]))
        time.sleep(0.1)
        qmp(pointer(a[0], a[1]))
        time.sleep(0.2)
        which = a[2] if len(a) > 2 else "left"
        if which not in ("left", "right", "middle"):
            die("the button is left, right or middle")
        qmp(button(True, which))
        time.sleep(0.1)
        qmp(button(False, which))
    elif op == "scroll":
        qmp(pointer(a[0], a[1]))
        wheel = "wheel-down" if int(a[2]) > 0 else "wheel-up"
        for _ in range(abs(int(a[2]))):
            qmp({"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": True, "button": wheel}}]}})
            qmp({"execute": "input-send-event", "arguments": {"events": [{"type": "btn", "data": {"down": False, "button": wheel}}]}})
            time.sleep(0.05)
    elif op == "type":
        for ch in a[0]:
            if ch.isupper(): k = ["shift", ch.lower()]
            elif ch in SHIFTED: k = ["shift", SHIFTED[ch]]
            else: k = [KEYS.get(ch, ch)]
            qmp(keys(k))
            time.sleep(0.03)
    elif op == "key":
        qmp(keys(a))
    else:
        die("unknown: " + op)

if __name__ == "__main__":
    main()

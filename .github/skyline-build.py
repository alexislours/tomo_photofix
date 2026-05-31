#!/usr/bin/env python3
import os
import pty
import select
import sys
import termios
import time

PROMPT = b"Would you like to install it?"
THROTTLE = 1.0
MAX_LINE = 4096


def main():
    cmd = sys.argv[1:]
    if not cmd:
        sys.exit("usage: skyline-build.py <command> [args...]")

    pid, fd = pty.fork()
    if pid == 0:
        os.execvp(cmd[0], cmd)
        os._exit(127)

    try:
        attrs = termios.tcgetattr(fd)
        attrs[3] &= ~termios.ECHO
        termios.tcsetattr(fd, termios.TCSANOW, attrs)
    except (termios.error, OSError):
        pass

    answered = False
    seen = b""
    line = bytearray()
    esc = 0
    last_emit = 0.0

    def flush():
        os.write(1, bytes(line) + b"\n")

    while True:
        try:
            ready, _, _ = select.select([fd], [], [], 1.0)
        except OSError:
            break
        if not ready:
            continue
        try:
            data = os.read(fd, 4096)
        except OSError:
            break
        if not data:
            break

        if not answered:
            seen = (seen + data)[-4096:]
            if PROMPT in seen:
                os.write(fd, b"y\n")
                answered = True

        for b in data:
            if esc == 1:
                esc = 2 if b == 0x5B else 0
                continue
            if esc == 2:
                if 0x40 <= b <= 0x7E:
                    esc = 0
                continue
            if b == 0x1B:
                esc = 1
            elif b == 0x0A:
                if line:
                    flush()
                line.clear()
                last_emit = time.monotonic()
            elif b == 0x0D:
                now = time.monotonic()
                if line and now - last_emit >= THROTTLE:
                    flush()
                    last_emit = now
                line.clear()
            elif b == 0x08:
                pass
            elif len(line) < MAX_LINE:
                line.append(b)

    if line:
        flush()

    _, status = os.waitpid(pid, 0)
    if os.WIFSIGNALED(status):
        sys.exit(128 + os.WTERMSIG(status))
    sys.exit(os.WEXITSTATUS(status))


if __name__ == "__main__":
    main()

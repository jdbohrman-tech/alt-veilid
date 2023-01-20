#!/usr/bin/env python3

import sys
import os
import io
import argparse
import subprocess
import signal
import time
from threading import Thread

if sys.version_info < (3, 0, 0):
    print(__file__ + ' requires Python 3, while Python ' +
          str(sys.version[0] + ' was detected. Terminating. '))
    sys.exit(1)

script_dir = os.path.dirname(os.path.realpath(__file__))
veilid_server_exe_debug = os.path.join(script_dir, '..',
                                       'target', 'debug', 'veilid-server')
veilid_server_exe_release = os.path.join(
    script_dir, '..', 'target', 'release', 'veilid-server')
main_process = None
subindex_processes = []

try:
    # Python 3, open as binary, then wrap in a TextIOWrapper with write-through.
    sys.stdout = io.TextIOWrapper(
        open(sys.stdout.fileno(), 'wb', 0), write_through=True)
    sys.stderr = io.TextIOWrapper(
        open(sys.stderr.fileno(), 'wb', 0), write_through=True)
except TypeError:
    # Python 2
    sys.stdout = os.fdopen(sys.stdout.fileno(), 'w', 0)
    sys.stderr = os.fdopen(sys.stderr.fileno(), 'w', 0)


def tee(prefix, infile, *files):
    """Print `infile` to `files` in a separate thread."""

    def fanout(prefix, infile, *files):
        with infile:
            for line in iter(infile.readline, b""):
                for f in files:
                    f.write(prefix + line)
                    f.flush()

    t = Thread(target=fanout, args=(prefix, infile,) + files)
    t.daemon = True
    t.start()
    return t


def read_until_interface_dial_info(proc, proto):

    interface_dial_info_str = b"Local Dial Info: "
    for ln in iter(proc.stdout.readline, ""):
        sys.stdout.buffer.write(ln)
        sys.stdout.flush()

        idx = ln.find(interface_dial_info_str)
        if idx != -1:
            idx += len(interface_dial_info_str)
            di = ln[idx:]
            if b"@"+bytes(proto)+b"|" in di:
                return di.decode("utf-8").strip()

    return None


class CleanChildProcesses:
    def __enter__(self):
        os.setpgrp()  # create new process group, become its leader

    def __exit__(self, type, value, traceback):
        try:
            os.killpg(0, signal.SIGKILL)  # kill all processes in my group
        except KeyboardInterrupt:
            pass


def main():
    threads = []

    # Parse arguments
    parser = argparse.ArgumentParser(description='Run veilid servers locally')
    parser.add_argument("count", type=int,
                        help='number of instances to run')
    parser.add_argument("--release", action='store_true',
                        help='use release mode build')
    parser.add_argument("--log_trace", action='store_true',
                        help='use trace logging')
    parser.add_argument("--log_info", action='store_true',
                        help='use info logging')
    parser.add_argument("-w", "--wait-for-debug", action='append',
                        help='specify subnode index to wait for the debugger')
    parser.add_argument("--config-file", type=str,
                        help='configuration file to specify for the bootstrap node')
    parser.add_argument("--protocol", type=str, default="udp",
                        help='default protocol to choose for dial info')
    args = parser.parse_args()

    if args.count < 1:
        print("Must specify more than one instance")
        sys.exit(1)

    veilid_server_exe = None
    if args.release:
        veilid_server_exe = veilid_server_exe_release
    else:
        veilid_server_exe = veilid_server_exe_debug

    base_args = [veilid_server_exe]
    if args.log_info:
        pass
    elif args.log_trace:
        base_args.append("--trace")
    else:
        base_args.append("--debug")

    if args.config_file:
        base_args.append("--config-file={}".format(args.config_file))

    # Run primary node and get node id
    main_args = base_args.copy()
    if args.wait_for_debug and ("0" in args.wait_for_debug):
        main_args.append("--wait-for-debug")

    print("Running main node: {}".format(str(main_args)))
    main_proc = subprocess.Popen(
        main_args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(">>> MAIN NODE PID={}".format(main_proc.pid))

    main_di = read_until_interface_dial_info(
        main_proc, bytes(args.protocol, 'utf-8'))

    print(">>> MAIN DIAL INFO={}".format(main_di))

    threads.append(
        tee(b"Veilid-0: ", main_proc.stdout, open("/tmp/veilid-0-out", "wb"),
            getattr(sys.stdout, "buffer", sys.stdout))
    )

    # Run all secondaries and add primary to bootstrap
    for n in range(1, args.count):

        time.sleep(1)

        sub_args = base_args.copy()
        sub_args.append("--subnode-index={}".format(n))
        sub_args.append("--bootstrap-nodes={}".format(main_di))
        if args.wait_for_debug and (str(n) in args.wait_for_debug):
            sub_args.append("--wait-for-debug")

        print("Running subnode {}: {}".format(n, str(sub_args)))

        sub_proc = subprocess.Popen(
            sub_args, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

        print(">>> SUBNODE {} NODE PID={}".format(n, sub_proc.pid))

        threads.append(
            tee("Veilid-{}: ".format(n).encode("utf-8"), sub_proc.stdout, open("/tmp/veilid-{}-out".format(n), "wb"),
                getattr(sys.stdout, "buffer", sys.stdout))
        )

    for t in threads:
        t.join()  # wait for IO completion

    sys.exit(0)


if __name__ == "__main__":

    with CleanChildProcesses():
        main()

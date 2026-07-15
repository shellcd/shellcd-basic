#!/usr/bin/env python3
"""Example forced-command policy; install root-owned and mode 0755."""

import os
import shlex
import sys

ALLOWED_SCRIPTS = {"/opt/shellcd/scripts/deploy-api.sh"}
EXPECTED_FLAGS = [
    "--caller-email",
    "--gitlab-user-login",
    "--project-path",
    "--pipeline-id",
    "--job-id",
    "--commit-sha",
]


def reject(message: str) -> "None":
    print(f"shellcd-dispatcher: {message}", file=sys.stderr)
    raise SystemExit(126)


try:
    arguments = shlex.split(os.environ.get("SSH_ORIGINAL_COMMAND", ""), posix=True)
except ValueError:
    reject("malformed command")

required_count = 1 + 2 * len(EXPECTED_FLAGS)
if len(arguments) not in {required_count, required_count + 1}:
    reject("unexpected argument count")
if arguments[0] not in ALLOWED_SCRIPTS:
    reject("script is not allowed")
if arguments[1:required_count:2] != EXPECTED_FLAGS:
    reject("unexpected argument schema")
if any("\0" in value for value in arguments):
    reject("invalid argument")

environment = {
    "PATH": "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
    "LANG": "C.UTF-8",
}
os.execve(arguments[0], arguments, environment)

# Simple test

This is the smallest end-to-end `shellcd-basic` test. It connects to a temporary Alpine SSH server,
runs an approved remote script, prints a message, and creates `/tmp/shellcd-simple-test`.

## Run

From the repository root:

```bash
sudo ./tests/simple-test/index.sh [SHELLCD_IMAGE]
```

Example:

```bash
sudo ./tests/simple-test/index.sh kimc1992/shellcd-basic:0.0.2
```

`SHELLCD_IMAGE` defaults to `kimc1992/shellcd-basic:0.0.2`.

## Pass criteria

- SSH certificate authentication succeeds.
- The remote script exits with status `0`.
- `/tmp/shellcd-simple-test` exists in the SSH server.
- Remote stdout is visible in the test output.

This test does not mount the host Docker socket into the SSH server.

# Artifact extraction and nginx HUP test

This test models an nginx static-site deployment. It starts a live nginx container with a shared
site volume, extracts `/usr/share/nginx/html` from the requested artifact image into that volume,
and sends `SIGHUP` to nginx so it reloads without replacing the running container.

## Run

From the repository root:

```bash
sudo ./tests/artifact-extract-nginx-hup-test/index.sh [SHELLCD_IMAGE] [ARTIFACT_IMAGE]
```

Example:

```bash
sudo ./tests/artifact-extract-nginx-hup-test/index.sh \
  kimc1992/shellcd-basic:0.0.2 \
  nginx:1.27-alpine
```

Defaults:

- `SHELLCD_IMAGE`: `kimc1992/shellcd-basic:0.0.2`
- `ARTIFACT_IMAGE`: `nginx:1.27-alpine`

## Pass criteria

- The extracted `index.html` exists and is non-empty in the shared volume.
- The nginx container remains running.
- nginx logs contain confirmation that signal 1 (`SIGHUP`) was received.
- The temporary extraction container is removed.

The test removes the live nginx container and its site volume during cleanup. It mounts the host
Docker socket into the SSH server and must only be used in an isolated local test environment.

# Artifact extraction test

This test demonstrates deploying files from a container image without starting that image as the
application. The remote script creates a temporary container and extracts:

```text
/usr/share/nginx/html
```

into a timestamped directory below `/srv/shellcd-test/artifacts/releases`. It then atomically
updates the `current` symbolic link.

## Run

From the repository root:

```bash
sudo ./tests/artifact-extract-test/index.sh [SHELLCD_IMAGE] [ARTIFACT_IMAGE]
```

Example:

```bash
sudo ./tests/artifact-extract-test/index.sh \
  kimc1992/shellcd-basic:0.0.2 \
  nginx:1.27-alpine
```

Defaults:

- `SHELLCD_IMAGE`: `kimc1992/shellcd-basic:0.0.2`
- `ARTIFACT_IMAGE`: `nginx:1.27-alpine`

## Pass criteria

- The artifact image is pulled successfully.
- `current/index.html` exists and is non-empty.
- The extracted page contains an nginx marker.
- The temporary extraction container is removed.

The source folder is intentionally fixed in `deploy.sh`; only the reviewed image reference is
provided as the single `shellcd-basic` script argument. This test mounts the host Docker socket into
the SSH server.

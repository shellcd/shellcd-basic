# Docker Compose test

This test verifies a tag-based Compose deployment through `shellcd-basic`. The remote deployment
script validates the requested tag, atomically updates `/srv/shellcd-test/.env`, and runs:

```bash
docker compose up --detach
```

The application is a long-running Alpine container described by `compose.yml`.

## Run

From the repository root:

```bash
sudo ./tests/docker-compose-test/index.sh [SHELLCD_IMAGE] [DEPLOY_TAG]
```

Example:

```bash
sudo ./tests/docker-compose-test/index.sh kimc1992/shellcd-basic:0.0.2 3.21
```

Defaults:

- `SHELLCD_IMAGE`: `kimc1992/shellcd-basic:0.0.2`
- `DEPLOY_TAG`: `3.21`

The tag may contain ASCII letters, digits, `.`, `_`, and `-`.

## Pass criteria

- The `.env` file contains the requested `IMAGE_TAG`.
- `docker compose up -d` succeeds remotely.
- The running `shellcd-compose-test-app` container reports `alpine:<DEPLOY_TAG>` as its image.

This test mounts the host Docker socket into the SSH server. Use it only in an isolated local test
environment.

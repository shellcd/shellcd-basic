# Container integration tests

These tests run the published `shellcd-basic` image against a temporary Alpine SSH server. Each
test generates an Ed25519 CA, a short-lived OpenSSH user certificate, a client key, and a trusted
host key. The image runs as its default non-root `shellcd` user.

## Prerequisites

- Docker with access to `/var/run/docker.sock`;
- `ssh-keygen` on the host;
- permission to pull the images used by a test;
- root or equivalent Docker access when required by the local Docker installation.

The test server image includes the Docker CLI and Docker Compose plugin. Host Docker Compose is not
required.

## Tests

| Test | Deployment behavior |
|---|---|
| [Simple](simple-test/README.md) | Runs a remote echo and creates a directory. |
| [Docker Compose](docker-compose-test/README.md) | Updates an image tag and runs `docker compose up -d`. |
| [Artifact extraction](artifact-extract-test/README.md) | Extracts `/usr/share/nginx/html` from an image. |
| [Artifact extraction and nginx HUP](artifact-extract-nginx-hup-test/README.md) | Replaces nginx files and sends `SIGHUP`. |

Run all tests individually from the repository root. For example:

```bash
sudo ./tests/simple-test/index.sh kimc1992/shellcd-basic:0.0.2
```

Every test removes the containers, networks, volumes, temporary server image, keys, and
certificates that it creates. Pulled public images are retained so an existing local image is never
removed unexpectedly.

## Security note

The Compose and artifact tests mount the host Docker socket into the isolated SSH test server.
Docker socket access is equivalent to host root access. This arrangement is strictly for local
testing and is not a production deployment recommendation.

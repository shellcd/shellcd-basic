# shellcd-basic

The small, SSH-key-based edition of ShellCD for running one validated deployment script from
GitLab CI.

> **Project status:** `shellcd-basic` is available now. A future `shellcd-secure` edition is planned
> for environments that require GitLab OIDC, short-lived OpenSSH certificates, and centralized
> issuance policy. That secure edition is not implemented yet.

## What Basic provides

- strict `known_hosts` verification;
- public-key-only, non-interactive OpenSSH execution;
- a validated script path below `/opt/shellcd/scripts/`;
- safe escaping for GitLab metadata and one optional script argument;
- unchanged remote stdout/stderr and exact remote exit codes where OpenSSH permits;
- secure SSH client defaults with forwarding, agents, passwords, and PTY disabled.

`shellcd-basic` still uses a long-lived private key stored as a protected GitLab file variable. The
remote server must restrict that key and enforce the final command authorization boundary.

## Quick GitLab example

Create these protected, file-type CI/CD variables in GitLab:

- `SHELLCD_PRIVATE_KEY_FILE`: the deployment user's SSH private key;
- `SHELLCD_KNOWN_HOSTS_FILE`: the trusted SSH host-key entry.

GitLab writes each value to a temporary file and exposes its path through the variable. The
variable names therefore map directly to the file-path settings expected by `shellcd-basic`; do
not store the private key in a regular environment variable.

```yaml
deploy:
  image:
    name: kimc1992/shellcd-basic:latest
    entrypoint: [""]

  variables:
    SHELLCD_HOST: "deploy.example.com"
    SHELLCD_SSH_USER: "deploy"
    SHELLCD_SCRIPT: "/opt/shellcd/scripts/deploy-api.sh"

  before_script:
    - chmod 600 "$SHELLCD_PRIVATE_KEY_FILE"

  script:
    - shellcd-basic run
```

`GITLAB_USER_EMAIL` and the other supported GitLab metadata variables are read automatically. Pin
a reviewed version tag or image digest for production. Provision `known_hosts` from a trusted
source; do not discover the host key with unauthenticated `ssh-keyscan` in the deployment job.

## Documentation

- [English setup and usage](https://github.com/shellcd/.github/blob/main/README.md)
- [한국어 설치 및 사용법](https://github.com/shellcd/.github/blob/main/README.ko.md)
- [Production hardening and shellcd-secure roadmap](https://github.com/shellcd/.github/blob/main/PRODUCTION.md)
- [Docker Hub image](https://hub.docker.com/repository/docker/kimc1992/shellcd-basic/general)

## Basic now, Secure later

| | `shellcd-basic` | Future `shellcd-secure` |
|---|---|---|
| CI authentication | Long-lived SSH private key | GitLab OIDC identity |
| SSH credential | Existing key | Short-lived OpenSSH certificate |
| Credential issuance | Manual provisioning and rotation | Policy-controlled issuance |
| Availability | Implemented | Planned; design may change |

Both editions require trusted host-key verification, a least-privilege deployment account, and
server-side command restrictions. Short-lived credentials reduce key exposure; they do not make an
unrestricted remote account safe.

## Development

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build --release
```

Apache-2.0 licensed. See [LICENSE](LICENSE).

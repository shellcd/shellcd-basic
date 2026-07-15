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

```yaml
deploy:
  image:
    name: kimc1992/shellcd-basic:latest
    entrypoint: [""]
  script:
    - shellcd-basic run
```

Pin a reviewed version tag or image digest for production.

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

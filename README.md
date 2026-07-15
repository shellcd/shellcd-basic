# shellcd-basic

A small, hardened GitLab CI tool that connects to one Linux server over SSH and runs one validated
script below `/opt/shellcd/scripts/`.

```yaml
deploy:
  image:
    name: kimc1992/shellcd-basic:latest
    entrypoint: [""]
  script:
    - shellcd-basic run
```

## Documentation

- [English setup and usage](https://github.com/shellcd/.github/README.md)
- [한국어 설치 및 사용법](https://github.com/shellcd/.github/README.ko.md)
- [Production hardening](https://github.com/shellcd/.github/PRODUCTION.md)
- [Docker Hub image](https://hub.docker.com/repository/docker/kimc1992/shellcd-basic/general)

`shellcd-basic` validates the destination and script, requires strict `known_hosts` verification,
disables interactive SSH behavior, streams remote output, and returns the remote exit code. The
remote server must still enforce the final authorization policy.

## Development

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build --release
```

Apache-2.0 licensed. See [LICENSE](LICENSE).

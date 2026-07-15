#!/bin/sh
set -eu

if [ -S /var/run/docker.sock ]; then
    docker_group=$(awk -F: -v gid="$DOCKER_GID" '$3 == gid { print $1; exit }' /etc/group)
    if [ -z "$docker_group" ]; then
        docker_group=docker-host
        addgroup -g "$DOCKER_GID" "$docker_group"
    fi
    addgroup deploy "$docker_group"
fi

if [ -n "${NGINX_CONTAINER:-}" ]; then
    printf '%s\n' "$NGINX_CONTAINER" >/run/nginx-container-name
    chown deploy:deploy /run/nginx-container-name
fi

exec /usr/sbin/sshd \
    -D \
    -e \
    -o HostKey=/run/test-secrets/host_key \
    -o TrustedUserCAKeys=/run/test-secrets/user_ca.pub \
    -o AuthorizedKeysFile=none \
    -o PasswordAuthentication=no \
    -o KbdInteractiveAuthentication=no \
    -o PermitRootLogin=no \
    -o AllowUsers=deploy

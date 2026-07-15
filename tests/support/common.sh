#!/bin/sh

common_start() {
    test_name=$1
    remote_script=$2
    docker_access=${3:-false}

    test_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
    common_secrets=$(mktemp -d)
    common_suffix=$$
    common_network="shellcd-$test_name-$common_suffix"
    common_server="shellcd-$test_name-server-$common_suffix"
    common_credentials="shellcd-$test_name-credentials-$common_suffix"
    common_server_image="shellcd-test-server:$common_suffix"

    ssh-keygen -q -t ed25519 -N '' -f "$common_secrets/user_ca"
    ssh-keygen -q -t ed25519 -N '' -f "$common_secrets/client_key"
    ssh-keygen -q \
        -s "$common_secrets/user_ca" \
        -I "$test_name" \
        -n deploy \
        -V -5m:+5m \
        "$common_secrets/client_key.pub"
    ssh-keygen -q -t ed25519 -N '' -f "$common_secrets/host_key"
    awk '{ print "shellcd-target " $1 " " $2 }' \
        "$common_secrets/host_key.pub" >"$common_secrets/known_hosts"

    docker pull "$SHELLCD_IMAGE"
    docker build --tag "$common_server_image" "$test_root/support"
    docker network create "$common_network" >/dev/null
    docker volume create "$common_credentials" >/dev/null

    set -- docker run --detach \
        --name "$common_server" \
        --hostname shellcd-target \
        --network "$common_network" \
        --network-alias shellcd-target \
        --volume "$common_secrets:/run/test-secrets:ro" \
        --volume "$remote_script:/opt/shellcd/scripts/deploy-test.sh:ro"

    if [ "$docker_access" = true ]; then
        set -- "$@" \
            --env "DOCKER_GID=$(stat -c %g /var/run/docker.sock)" \
            --volume /var/run/docker.sock:/var/run/docker.sock
    fi

    "$@" "$common_server_image" >/dev/null

    attempt=0
    until docker exec "$common_server" nc -z 127.0.0.1 22; do
        attempt=$((attempt + 1))
        if [ "$attempt" -ge 30 ]; then
            docker logs "$common_server"
            printf '%s\n' 'SSH server did not become ready' >&2
            return 1
        fi
        sleep 1
    done

    docker run --rm \
        --user 0:0 \
        --entrypoint /bin/sh \
        --volume "$common_secrets:/source:ro" \
        --volume "$common_credentials:/credentials" \
        "$SHELLCD_IMAGE" \
        -c 'cp /source/client_key /credentials/private_key &&
            cp /source/client_key-cert.pub /credentials/private_key-cert.pub &&
            cp /source/known_hosts /credentials/known_hosts &&
            chown shellcd:shellcd /credentials/private_key /credentials/private_key-cert.pub \
                /credentials/known_hosts &&
            chmod 600 /credentials/private_key &&
            chmod 644 /credentials/private_key-cert.pub /credentials/known_hosts'
}

common_run_shellcd() {
    script_arg=${1:-}
    set -- docker run --rm \
        --network "$common_network" \
        --volume "$common_credentials:/run/secrets:ro" \
        --env SHELLCD_HOST=shellcd-target \
        --env SHELLCD_SSH_USER=deploy \
        --env SHELLCD_SCRIPT=/opt/shellcd/scripts/deploy-test.sh \
        --env SHELLCD_PRIVATE_KEY_FILE=/run/secrets/private_key \
        --env SHELLCD_KNOWN_HOSTS_FILE=/run/secrets/known_hosts \
        --env GITLAB_USER_EMAIL=smoke-test@example.com

    if [ -n "$script_arg" ]; then
        set -- "$@" --env "SHELLCD_SCRIPT_ARG=$script_arg"
    fi

    "$@" "$SHELLCD_IMAGE" run
}

common_cleanup() {
    docker container rm --force "$common_server" >/dev/null 2>&1 || true
    docker volume rm "$common_credentials" >/dev/null 2>&1 || true
    docker network rm "$common_network" >/dev/null 2>&1 || true
    docker image rm --force "$common_server_image" >/dev/null 2>&1 || true
    rm -rf "$common_secrets"
}

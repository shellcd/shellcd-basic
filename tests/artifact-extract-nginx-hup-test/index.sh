#!/bin/sh
set -eu

SHELLCD_IMAGE=${1:-kimc1992/shellcd-basic:0.0.2}
ARTIFACT_IMAGE=${2:-nginx:1.27-alpine}
export SHELLCD_IMAGE
directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$directory/../support/common.sh"

site_volume="shellcd-nginx-site-$$"
nginx_container="shellcd-nginx-hup-target-$$"

cleanup() {
    docker container rm --force "$nginx_container" >/dev/null 2>&1 || true
    docker volume rm "$site_volume" >/dev/null 2>&1 || true
    common_cleanup
}
trap cleanup EXIT HUP INT TERM

common_start artifact-nginx-hup-test "$directory/deploy.sh" true
docker volume create "$site_volume" >/dev/null
docker run --detach \
    --name "$nginx_container" \
    --volume "$site_volume:/usr/share/nginx/html:ro" \
    nginx:1.27-alpine >/dev/null
docker container stop "$common_server" >/dev/null
docker container rm "$common_server" >/dev/null

docker run --detach \
    --name "$common_server" \
    --hostname shellcd-target \
    --network "$common_network" \
    --network-alias shellcd-target \
    --env "DOCKER_GID=$(stat -c %g /var/run/docker.sock)" \
    --env "NGINX_CONTAINER=$nginx_container" \
    --volume /var/run/docker.sock:/var/run/docker.sock \
    --volume "$common_secrets:/run/test-secrets:ro" \
    --volume "$directory/deploy.sh:/opt/shellcd/scripts/deploy-test.sh:ro" \
    --volume "$site_volume:/srv/nginx-html" \
    "$common_server_image" >/dev/null

docker exec --user 0:0 "$common_server" chown deploy:deploy /srv/nginx-html

attempt=0
until docker exec "$common_server" nc -z 127.0.0.1 22; do
    attempt=$((attempt + 1))
    [ "$attempt" -lt 30 ] || exit 1
    sleep 1
done

common_run_shellcd "$ARTIFACT_IMAGE"
docker exec "$common_server" test -s /srv/nginx-html/index.html
docker inspect --format '{{.State.Running}}' "$nginx_container" | grep -Fx true

attempt=0
until docker logs "$nginx_container" 2>&1 | grep -F 'signal 1 (SIGHUP) received'; do
    attempt=$((attempt + 1))
    [ "$attempt" -lt 10 ] || exit 1
    sleep 1
done

printf 'PASS: extracted %s and sent SIGHUP to nginx\n' "$ARTIFACT_IMAGE"

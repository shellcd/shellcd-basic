#!/bin/sh
set -eu

SHELLCD_IMAGE=${1:-kimc1992/shellcd-basic:0.0.2}
DEPLOY_TAG=${2:-3.21}
export SHELLCD_IMAGE
directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$directory/../support/common.sh"

case "$DEPLOY_TAG" in
    '' | *[!A-Za-z0-9._-]*)
        printf 'invalid image tag: %s\n' "$DEPLOY_TAG" >&2
        exit 2
        ;;
esac

cleanup() {
    docker container rm --force shellcd-compose-test-app >/dev/null 2>&1 || true
    common_cleanup
}
trap cleanup EXIT HUP INT TERM

common_start docker-compose-test "$directory/deploy.sh" true
docker cp "$directory/compose.yml" "$common_server:/srv/shellcd-test/compose.yml"
docker exec --user 0:0 "$common_server" \
    chown deploy:deploy /srv/shellcd-test/compose.yml
common_run_shellcd "$DEPLOY_TAG"

docker exec "$common_server" \
    grep -Fx "IMAGE_TAG=$DEPLOY_TAG" /srv/shellcd-test/.env
actual_image=$(docker inspect --format '{{.Config.Image}}' shellcd-compose-test-app)
test "$actual_image" = "alpine:$DEPLOY_TAG"

printf 'PASS: shellcd updated the tag and ran docker compose up -d (%s)\n' "$actual_image"

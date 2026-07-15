#!/bin/sh
set -eu

SHELLCD_IMAGE=${1:-kimc1992/shellcd-basic:0.0.2}
ARTIFACT_IMAGE=${2:-nginx:1.27-alpine}
export SHELLCD_IMAGE
directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$directory/../support/common.sh"

cleanup() {
    common_cleanup
}
trap cleanup EXIT HUP INT TERM

common_start artifact-extract-test "$directory/deploy.sh" true
common_run_shellcd "$ARTIFACT_IMAGE"

docker exec "$common_server" test -s /srv/shellcd-test/artifacts/current/index.html
docker exec "$common_server" grep -qi nginx /srv/shellcd-test/artifacts/current/index.html

printf 'PASS: extracted /usr/share/nginx/html from %s\n' "$ARTIFACT_IMAGE"

#!/bin/sh
set -eu

SHELLCD_IMAGE=${1:-kimc1992/shellcd-basic:0.0.2}
export SHELLCD_IMAGE
directory=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
. "$directory/../support/common.sh"

cleanup() {
    common_cleanup
}
trap cleanup EXIT HUP INT TERM

common_start simple-test "$directory/deploy.sh"
common_run_shellcd
docker exec "$common_server" test -d /tmp/shellcd-simple-test

printf '%s\n' 'PASS: simple remote echo and mkdir'

#!/bin/sh
set -eu

image=
for argument do
    image=$argument
done

case "$image" in
    '' | *[!A-Za-z0-9._:@/-]*)
        printf 'invalid artifact image: %s\n' "$image" >&2
        exit 2
        ;;
esac

release=/srv/shellcd-test/artifacts/releases/$(date +%s)
temporary_container=
cleanup() {
    if [ -n "$temporary_container" ]; then
        docker container rm --force "$temporary_container" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT HUP INT TERM

mkdir -p "$release"
docker pull "$image"
temporary_container=$(docker create "$image")
docker cp "$temporary_container:/usr/share/nginx/html/." "$release"
ln -sfn "$release" /srv/shellcd-test/artifacts/current

printf 'extracted %s:/usr/share/nginx/html to %s\n' "$image" "$release"

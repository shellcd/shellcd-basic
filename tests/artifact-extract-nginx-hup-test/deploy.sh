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

temporary_container=
cleanup() {
    if [ -n "$temporary_container" ]; then
        docker container rm --force "$temporary_container" >/dev/null 2>&1 || true
    fi
}
trap cleanup EXIT HUP INT TERM

docker pull "$image"
temporary_container=$(docker create "$image")
rm -rf /srv/nginx-html/*
docker cp "$temporary_container:/usr/share/nginx/html/." /srv/nginx-html
nginx_container=$(cat /run/nginx-container-name)
docker kill --signal HUP "$nginx_container" >/dev/null

printf 'extracted %s and reloaded %s\n' "$image" "$nginx_container"

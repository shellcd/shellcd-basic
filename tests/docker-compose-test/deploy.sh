#!/bin/sh
set -eu

image_tag=
for argument do
    image_tag=$argument
done

case "$image_tag" in
    '' | *[!A-Za-z0-9._-]*)
        printf 'invalid image tag: %s\n' "$image_tag" >&2
        exit 2
        ;;
esac

temporary=/srv/shellcd-test/.env.tmp
printf 'IMAGE_TAG=%s\n' "$image_tag" >"$temporary"
mv "$temporary" /srv/shellcd-test/.env

docker compose \
    --project-name shellcd-compose-test \
    --file /srv/shellcd-test/compose.yml \
    --env-file /srv/shellcd-test/.env \
    up --detach

printf 'updated tag and started Compose application: alpine:%s\n' "$image_tag"

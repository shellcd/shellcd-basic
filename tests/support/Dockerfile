FROM alpine:3.22

RUN apk add --no-cache docker-cli docker-cli-compose openssh-server \
    && adduser -D deploy \
    && echo 'deploy:disabled-password' | chpasswd \
    && mkdir -p /opt/shellcd/scripts /run/sshd /srv/shellcd-test \
    && chown deploy:deploy /srv/shellcd-test

COPY entrypoint.sh /usr/local/bin/shellcd-test-server
RUN chmod 755 /usr/local/bin/shellcd-test-server

EXPOSE 22
ENTRYPOINT ["/usr/local/bin/shellcd-test-server"]

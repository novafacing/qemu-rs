#!/bin/bash

set -e

# sudo -E dnf install -y cloud-utils

mkdir -p .github/rsrc/
rm .github/rsrc/id_* || true
if [ ! -f .github/rsrc/id_rsa ]; then
  ssh-keygen -C fedora@localhost -t rsa -q -f .github/rsrc/id_rsa -N ""
fi

KEY="$(cat .github/rsrc/id_rsa.pub)"
# password is "password"
# mkpasswd --method=SHA-512 --rounds=4096
PASSWORD='$6$rounds=4096$At.ZMrhUfvsFwTiG$VJ8aQCC3nr8SpUL99OHcWsR6BvlVur5qvKQHni8n5v1HxB0E3.2eLX0tbxq8nHv.JJb2cU5mXr8bAgogCd5Ke1'
cat <<EOF > .github/rsrc/user-data.yml
#cloud-config
bootcmd:
  - useradd -m -p ${PASSWORD} -s /bin/bash fedora
  - mkdir -p /home/fedora/.ssh
  - echo "${KEY}" >> /home/fedora/.ssh/authorized_keys
  - chown -R fedora:fedora /home/fedora/.ssh
  - chmod 700 /home/fedora/.ssh
  - chmod 600 /home/fedora/.ssh/authorized_keys
  - echo "fedora ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
EOF

cloud-localds .github/rsrc/seed.img .github/rsrc/user-data.yml
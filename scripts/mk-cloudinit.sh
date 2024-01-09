#!/bin/bash

set -e

# sudo -E dnf install -y cloud-utils

mkdir -p .github/rsrc/
# password is "password"
# mkpasswd --method=SHA-512 --rounds=4096
PASSWD="$6$rounds=4096$EhaOFVl.Hr626Zg2$mIqOEWTXg0U4cfIDDsYYLtqNMoCLRMVQfX4iZnlQTt.dnBoXetHdMzyGdY2MVOWGV18UowbFNSJowTHmBDb4z1"
cat <<EOF > .github/rsrc/user-data.yml
#cloud-config
users:
  - name: user
    passwd: ${PASSWD}
    lock_passwd: false
    groups: [sudo]
    shell: /bin/bash
EOF

cloud-localds .github/rsrc/seed.img .github/rsrc/user-data.yml
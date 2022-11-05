#!/bin/bash

find . -type d -maxdepth 1 -mindepth 1 | xargs -i bash -c \
'qemu_prog=$(printf {} | cut -d'/' -f2); qemu_func=$(printf $qemu_prog | sed "s/-/_/g"); echo $qemu_prog; echo $qemu_func; echo "memfd-exec = \"0.1.4\"" >> {}/Cargo.toml; cat main.rs.template | sed "s/QEMU_FUNCNAME/$qemu_func/g" | sed "s/QEMU_PROGNAME/$qemu_prog/g" > {}/src/main.rs'

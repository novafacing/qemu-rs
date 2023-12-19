# Templates

Template files for generating bins. Generated binary files with:

```sh
grep "required-features" Cargo.toml | awk '{print $4}' | grep -oE '(-|_|[a-z0-9])+' | xargs -i bash -c 'BINNAME=$(printf {} | sed -r '"'"'s/(.*)-softmmu/qemu-system-\1/'"'"' | sed -r '"'"'s/(.*)-linux-user/qemu-\1/'"'"'); TARGET={}; CONSTNAME=$(printf {} | awk '"'"'{ gsub(/-/, "_"); print "QEMU_"toupper($0);  }'"'"'); cp templates/bin.rs src/bin/$BINNAME.rs; sed -i "s/QEMU_BINARY_NAME/$BINNAME/g" src/bin/$BINNAME.rs; sed -i "s/QEMU_BINARY_CONST/$CONSTNAME/g" src/bin/$BINNAME.rs'
```
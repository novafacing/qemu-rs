#!/bin/bash

set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
REPO_ROOT=$(realpath "$SCRIPT_DIR/..")

CHECK_PATHS=(
    "$REPO_ROOT/qemu-plugin"
    "$REPO_ROOT/qemu-plugin-sys"
    "$REPO_ROOT/plugins/icount"
    "$REPO_ROOT/plugins/tiny"
    "$REPO_ROOT/plugins/tiny-system"
    "$REPO_ROOT/plugins/tracer"
)

FEATURES="plugin-api-v0,plugin-api-v1,plugin-api-v2,plugin-api-v3,plugin-api-v4,plugin-api-v5"
COMMON_ARGS=(
    "--mutually-exclusive-features=$FEATURES"
    "--at-least-one-of=$FEATURES"
    "--feature-powerset"
    "--exclude-no-default-features"
)

# Run cargo fmt from repo root to ensure workspace is detected
pushd "$REPO_ROOT" > /dev/null
cargo fmt --all --check
popd > /dev/null

for CHECK_PATH in "${CHECK_PATHS[@]}"; do
    MANIFEST_PATH="$CHECK_PATH/Cargo.toml"

    cargo +nightly hack --manifest-path "$MANIFEST_PATH" "${COMMON_ARGS[@]}" check --lib
    cargo +nightly hack --manifest-path "$MANIFEST_PATH" "${COMMON_ARGS[@]}" clippy --lib
done
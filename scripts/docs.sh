#!/bin/bash
# Script to
# Mirrors the generate-documentation job from GitHub Actions

# Change to the repository root directory (where this script's parent directory is located)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"

# the `--enable-index-page` flag creates an index.html file at the root of the docs
# the `-Zunstable-options` flag is needed to use `--enable-index-page`
# see https://github.com/rust-lang/cargo/issues/8229 for why `nightly` is needed
RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --no-deps --workspace

# Build UI and copy to docs
./scripts/build.sh release
./ui/embed-wasm.sh
mkdir -p target/doc/ui
cp -r ui/* target/doc/ui/

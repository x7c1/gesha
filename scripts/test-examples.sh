#!/usr/bin/env bash

set -xue

main () {
  cd ./examples/v3_0

  cargo fmt -- --check

  cargo clippy -- \
    --no-deps \
    --deny warnings

  cargo test
}

main

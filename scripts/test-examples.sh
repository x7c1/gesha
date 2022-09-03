#!/usr/bin/env bash

set -xue

main () {
  cd ./examples/v3.0

  cargo fmt -- --check

  cargo clippy -- \
    --no-deps \
    --allow clippy::derive_partial_eq_without_eq \
    --deny warnings

  cargo test
}

main

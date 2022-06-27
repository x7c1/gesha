#!/usr/bin/env bash

set -xue

main () {
  cd ./examples/v3.0
  cargo fmt
  cargo clippy -- --no-deps -D warnings
  cargo test
}

main

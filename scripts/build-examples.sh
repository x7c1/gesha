#!/usr/bin/env bash

set -xue

main () {
  cd ./examples/v3.0
  cargo build
}

main

#!/usr/bin/env bash

set -xue

main() {
  output="output/v3.0/components/pet.rs"
  expected="tests/v3.0/components/pet.rs"

  cargo run --bin gesha -- \
    generate-sample \
    --schema tests/v3.0/components/pet.yaml \
    --output "$output"

  rustfmt "$output"

  diff "$output" "$expected"
}

main

#!/usr/bin/env bash

set -xue

main() {
  # TODO: remove variables
  schema="examples/v3.0/components/pet.yaml"
  output="output/v3.0/components/pet.rs"

  # TODO: run within rust function
  if [ -f "$output" ]; then
    rm "$output"
  fi

  cargo run --bin gesha -- \
    test \
    --schema "$schema" \
    --output "$output"
}

main

#!/usr/bin/env bash

set -xue

main() {
  schema="examples/v3.0/components/pet.yaml"
  expected="examples/v3.0/components/pet.rs"
  output="output/v3.0/components/pet.rs"

  if [ -f "$output" ]; then
    rm "$output"
  fi

  cargo run --bin gesha -- \
    test \
    --schema "$schema" \
    --output "$output"

  diff "$output" "$expected"
}

main

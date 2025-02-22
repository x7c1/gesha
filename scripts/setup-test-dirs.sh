#!/usr/bin/env bash

set -xue

dirs=(
  "./output/v3.0/components/request_bodies"
  "./output/v3.0/components/schemas"
  "./output/v3.0/example"
)

for dir in "${dirs[@]}"; do
  mkdir -p "$dir"
done

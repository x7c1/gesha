#!/usr/bin/env bash

# Usage
# $ ./scripts/e2e-test.sh ./e2e-tests/handcraft/petstore/add_pet.test.ts
# $ ./scripts/e2e-test.sh ./e2e-tests/handcraft

args=$*

if ! [[ $args ]]; then
  args='./e2e-tests'
fi

set -xue

main () {
  cargo build

  log_path="logs/$(date '+%F').log"
  cargo run > "$log_path" &

  run_tests
}

run_tests() {
  sleep 1
  await_server

  # shellcheck disable=SC2086
  deno test --allow-net --allow-read $args
}

await_server() {
  while ! lsof -i:8080; do
    sleep 1
  done
}

exit_handler() {
  kill_server
}

kill_server() {
  server_pid=$(lsof -i:8080 -t)
  echo "stopping server process..."
  kill "$server_pid"
}

trap 'exit_handler' EXIT
main

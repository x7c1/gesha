#!/usr/bin/env bash

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
  deno test --allow-net ./e2e-tests/
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
#!/usr/bin/env bash

# Usage
# $ ./scripts/e2e-test.sh ./e2e-tests/handcraft/petstore/add_pet.test.ts
# $ ./scripts/e2e-test.sh ./e2e-tests/handcraft

args=$*
log_path="logs/$(date '+%F').log"

if ! [[ $args ]]; then
  args='./e2e-tests'
fi

set -xue

main () {
  cargo build
  cargo_run > "$log_path" &
  run_tests
}

cargo_run() {
  set +e
  cargo run --bin handcraft-app
  exit_code=$?
  echo "cargo_run($exit_code)" >> "$log_path"
}

run_tests() {
  sleep 1
  await_server

  # shellcheck disable=SC2086
  deno test --allow-net --allow-read $args
}

await_server() {
  while ! lsof -i:8080; do
    if is_cargo_run_failed; then
      exit 1
    fi;
    sleep 1
  done
}

is_cargo_run_failed() {
  if tail -1 "$log_path" | grep "cargo_run" ; then
    return 0
  fi
  return 1
}

exit_handler() {
  if ! is_cargo_run_failed; then
    kill_server
  fi;
}

kill_server() {
  server_pid=$(lsof -i:8080 -t)
  echo "stopping server process..."
  kill "$server_pid"
}

trap 'exit_handler' EXIT
main

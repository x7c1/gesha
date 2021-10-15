#!/usr/bin/env bash

set -xue

cargo run &

sleep 1

server_pid=$(lsof -i:8080 -t)
kill "$server_pid"

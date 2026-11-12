#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")"

fingerprint() {
  find src public preview/main.rs preview/Cargo.toml -type f -printf '%T@ %p\n' | sort | sha256sum
}

server_pid=''
stop_server() {
  if [ -n "$server_pid" ]; then
    kill "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
}
trap 'stop_server' EXIT
trap 'exit 130' INT TERM

while :; do
  cargo build --manifest-path preview/Cargo.toml --bin saptrans-preview
  ./preview/target/debug/saptrans-preview &
  server_pid=$!
  previous=$(fingerprint)
  current=$previous
  while kill -0 "$server_pid" 2>/dev/null; do
    sleep 1
    current=$(fingerprint)
    if [ "$current" != "$previous" ]; then
      stop_server
      server_pid=''
      break
    fi
  done
  if [ "$current" = "$previous" ]; then
    wait "$server_pid"
    exit $?
  fi
done

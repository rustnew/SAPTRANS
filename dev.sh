#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")"

# Charge les paramètres locaux de PostgreSQL et de l'application.
if [ -f .env ]; then
  set -a
  . ./.env
  set +a
fi

if [ -z "${DATABASE_URL:-}" ]; then
  echo "DATABASE_URL manque. Configurez .env à partir de .env.example." >&2
  exit 1
fi

if command -v pg_isready >/dev/null 2>&1 && ! pg_isready -h localhost -p 5432 -q; then
  echo "PostgreSQL ne répond pas sur localhost:5432. Démarrez-le avec : sudo pg_ctlcluster 16 main start" >&2
  exit 1
fi

# Réduit la pression mémoire et évite le SIGSEGV de rustc observé sur js-sys.
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
RUST_MIN_STACK=${RUST_MIN_STACK:-33554432}
DEV_RELOAD=1
export CARGO_BUILD_JOBS RUST_MIN_STACK DEV_RELOAD

fingerprint() {
  find Cargo.toml src public migrations scripts -type f -printf '%T@ %p\n' | sort | sha256sum
  if [ -f SAPTRANS_Extraction_Ancien_Site_2026-11-09.html ]; then
    stat -c '%Y %n' SAPTRANS_Extraction_Ancien_Site_2026-11-09.html
  fi
  if [ -f Cargo.lock ]; then
    stat -c '%Y %n' Cargo.lock
  fi
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

echo "SAPTRANS : frontend Leptos SSR + backend Actix sur http://localhost:3000"
echo "Le navigateur se recharge après chaque modification des sources, styles ou médias."

while :; do
  if [ -f SAPTRANS_Extraction_Ancien_Site_2026-11-09.html ]; then
    python3 scripts/extract_legacy_content.py
  elif [ ! -s public/data/legacy-content.json ]; then
    echo "Le contenu local public/data/legacy-content.json manque et l’archive source est absente." >&2
    exit 1
  fi
  cargo build --bin saptrans-server
  ./target/debug/saptrans-server &
  server_pid=$!
  previous=$(fingerprint)
  current=$previous
  while kill -0 "$server_pid" 2>/dev/null; do
    sleep 0.7
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

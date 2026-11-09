#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")"
cargo run --manifest-path preview/Cargo.toml --bin saptrans-preview

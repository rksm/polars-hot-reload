#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

(trap 'kill 0' SIGINT; \
 bash -c 'cargo run --features reload' & \
 bash -c 'cargo watch -w lib -x "build -p lib"')

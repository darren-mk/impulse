#!/usr/bin/env bash
set -e

npx @tailwindcss/cli -i ./src/input.css -o ./static/output.css --watch &
cargo watch -x run
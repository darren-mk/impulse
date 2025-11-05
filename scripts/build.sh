#!/usr/bin/env bash
set -e  # exit immediately on error

pnpx @tailwindcss/cli -i ./src/input.css -o ./static/output.css --minify
cargo build --release
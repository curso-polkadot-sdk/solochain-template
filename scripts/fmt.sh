#!/bin/sh

# check for required programs
command -v dirname > /dev/null 2>&1 || { echo >&2 "'dirname' not found"; exit 1; }
command -v cargo > /dev/null 2>&1 || { echo >&2 "'cargo' not found"; exit 1; }
command -v dprint > /dev/null 2>&1 || { echo >&2 "'dprint' not found"; exit 1; }

# go to project root directory
cd -- "$(dirname "${0}")" || exit 1
cd .. || exit 1

# format *.rs files
cargo +nightly fmt --all || { echo >&2 "'cargo +nightly fmt --all' failed"; exit 1; }

# format *.toml and *.md files
dprint fmt || { echo >&2 "'dprint fmt' failed"; exit 1; }

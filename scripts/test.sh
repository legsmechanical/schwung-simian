#!/usr/bin/env bash
# Build and run the host-side tests natively (no device, no Docker).
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$REPO_ROOT"
mkdir -p build

# Regenerate first: the tests compare what the wrapper serves against what the
# generators wrote, so running them against a stale module.json would report a
# drift that a build was about to fix anyway.
if command -v node >/dev/null 2>&1; then
    node tools/gen_module_json.mjs
    node tools/gen_factory_bank.mjs
else
    echo "WARNING: node not found - generators NOT run"
fi

CXX_BIN="${CXX:-c++}"
"$CXX_BIN" -std=c++14 -O2 -Wall -Wno-comment -Isrc/dsp \
    tests/simian_test.cpp -o build/simian_test -lm
./build/simian_test

# The widget lives in JS and the pad table lives in C; only node can compare
# them. A drift here draws the wrong drum and nothing else notices.
if command -v node >/dev/null 2>&1; then
    node tests/widget.test.mjs
fi

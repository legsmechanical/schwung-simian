#!/usr/bin/env bash
# Render an audition WAV of the module (build/simian_demo.wav) so a change can
# be heard. Native build, no device needed.
#
#   ./scripts/render.sh                 two bars of kit 0
#   ./scripts/render.sh out.wav 12      kit 12
#   ./scripts/render.sh out.wav all     every one of the 35 kits
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$REPO_ROOT"
mkdir -p build
"${CXX:-c++}" -std=c++14 -O2 -Wno-comment -Isrc/dsp \
    tests/render_wav.cpp -o build/simian_render -lm
exec ./build/simian_render "${1:-build/simian_demo.wav}" "${2:-0}"

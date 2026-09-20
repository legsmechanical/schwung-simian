#!/usr/bin/env bash
# Build the SIMIAN module for Schwung (Move, ARM64).
#
# Auto-uses Docker for cross-compilation. Set CROSS_PREFIX to skip Docker.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
IMAGE_NAME="move-anything-builder"
ID="simian"

if [ -z "$CROSS_PREFIX" ] && [ ! -f "/.dockerenv" ]; then
    echo "=== SIMIAN Module Build (via Docker) ==="
    if ! docker image inspect "$IMAGE_NAME" &>/dev/null; then
        echo "Building Docker image (first time only)..."
        docker build -t "$IMAGE_NAME" -f "$SCRIPT_DIR/Dockerfile" "$REPO_ROOT"
    fi
    # Both generators run OUTSIDE the container (no node in the build image).
    if command -v node >/dev/null 2>&1; then
        # The host reads module.json's chain_params at synth load to build the
        # slot param table modulation and patches resolve against; a stale copy
        # is a set of silently dead destinations.
        node "$REPO_ROOT/tools/gen_module_json.mjs" || exit 1
        # The factory bank is indexed by VOICE_PARAMS[] position, so it must be
        # regenerated whenever a parameter is added or moved — otherwise every
        # value past the insertion point loads into the wrong slot, and nothing
        # fails to compile.
        node "$REPO_ROOT/tools/gen_factory_bank.mjs" || exit 1
    else
        echo "WARNING: node not found - module.json and factory_bank.h NOT regenerated"
    fi
    echo "Running build..."
    docker run --rm \
        -v "$REPO_ROOT:/build" \
        -u "$(id -u):$(id -g)" \
        -w /build \
        "$IMAGE_NAME" \
        ./scripts/build.sh
    echo "=== Done ==="
    exit 0
fi

CROSS_PREFIX="${CROSS_PREFIX:-aarch64-linux-gnu-}"
cd "$REPO_ROOT"

echo "=== Building SIMIAN Module ==="
echo "Cross prefix: $CROSS_PREFIX"

# Wipe dist first: every file below is regenerated from src, so anything
# surviving here came from an older build and would ship unnoticed.
rm -rf "dist/$ID"
mkdir -p build "dist/$ID"

# -Wno-comment: the generated Faust headers contain '/*' inside comments.
# -ffast-math is NOT used: the reverb and the limiter depend on ordinary IEEE
# rounding, and a denormal flush changes the tail. Measure before reaching
# for it.
echo "Compiling DSP plugin..."
${CROSS_PREFIX}g++ -g -O3 -shared -fPIC -std=c++14 -Wno-comment \
    src/dsp/simian_plugin.cpp \
    -o build/dsp.so \
    -Isrc/dsp \
    -lm

echo "Packaging..."
cat src/module.json > "dist/$ID/module.json"
[ -f src/help.json ] && cat src/help.json > "dist/$ID/help.json"
cat src/ui.js > "dist/$ID/ui.js"
cat build/dsp.so > "dist/$ID/dsp.so"
chmod +x "dist/$ID/dsp.so"

cd dist
tar -czf "$ID-module.tar.gz" "$ID/"
cd ..

echo ""
echo "=== Build Complete ==="
ls -la "dist/$ID"
echo "Tarball: dist/$ID-module.tar.gz"

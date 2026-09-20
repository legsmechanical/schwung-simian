#!/usr/bin/env bash
# Install the SIMIAN module to Move over the USB-ethernet tether (or WiFi).
# Set MOVE_HOST to override (default ableton@move.local).
#
# A dsp.so swap loads fresh on the next instantiation; if SIMIAN is loaded,
# swap it out and back in, or restart the host.
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"
ID="simian"
MOVE_HOST="${MOVE_HOST:-ableton@move.local}"
DEST="/data/UserData/schwung/modules/sound_generators/$ID"

cd "$REPO_ROOT"

if [ ! -d "dist/$ID" ]; then
    echo "Error: dist/$ID not found. Run ./scripts/build.sh first."
    exit 1
fi

echo "=== Installing SIMIAN Module -> $MOVE_HOST ==="
ssh "$MOVE_HOST" "mkdir -p $DEST"
# Copy to a temp name and atomic mv. Scp'ing directly over a live dlopen'd
# dsp.so overwrites the mapped inode in place and wedges the host.
for f in "dist/$ID/"*; do
    fn="$(basename "$f")"
    scp -q "$f" "$MOVE_HOST:$DEST/$fn.new"
    ssh "$MOVE_HOST" "mv -f '$DEST/$fn.new' '$DEST/$fn' && chmod a+rw '$DEST/$fn'"
done

echo "=== Installed to $DEST ==="

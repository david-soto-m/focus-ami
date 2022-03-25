#!/bin/sh

export MESON_BUILD_ROOT="$1"
export MESON_SOURCE_ROOT="$2"
# This exports are for meson, hidden args of the cargo command
export CARGO_TARGET_DIR="$MESON_BUILD_ROOT"/target
export CARGO_HOME="$MESON_BUILD_ROOT"/cargo-home
export OUTPUT="$3"
export APP_BIN="$4"

echo $OUTPUT
echo "RELEASE MODE"
cargo build --manifest-path \
    "$MESON_SOURCE_ROOT"/Cargo.toml --release && \
    cp "$CARGO_TARGET_DIR"/release/"$APP_BIN" "$OUTPUT"

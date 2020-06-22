#!/usr/bin/env bash
set -e # error -> trap -> exit
SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir
EXAMPLE=web_geo_viewer
cd $SRCDIR # ensure this script can be run from anywhere

# When using $CARGO_TARGET_DIR -> binary is located in different folder
# Necessary to locate build files for wasm-bindgen

# release or debug
MODE=release
TARGET_DIR=$PWD/target/wasm32-unknown-unknown/$MODE
cargo build --target wasm32-unknown-unknown --${MODE}
wasm-bindgen --target web --no-typescript --out-dir $PWD/static/ --out-name wasm $TARGET_DIR/$EXAMPLE.wasm

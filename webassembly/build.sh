#!/usr/bin/env sh

echo "Assembling stuff into wasm."

cargo build --target=wasm32-unknown-emscripten --release
rustc --target=wasm32-unknown-emscripten ../src/main.rs -L ../target/wasm32-unknown-emscripten/release/deps

if [ "$1" = "serve" ]; then
    python -m SimpleHTTPServer
fi


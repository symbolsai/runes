#!/bin/bash
wasm-pack test --node
wasm-pack build --release --target nodejs --scope canonicxyz
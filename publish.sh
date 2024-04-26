#!/bin/bash
./build.sh
npm run docs
wasm-pack publish --target nodejs --access public
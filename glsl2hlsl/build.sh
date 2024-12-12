#!/usr/bin/env bash

if [ "$1" = "dev" ]; then
  MODE="--dev"
  WEBPACK_MODE="development"
  ARGS="--features console_error_panic_hook"
elif [ "$1" = "prod" ]; then
  MODE="--release"
  WEBPACK_MODE="production"
else
  echo "Invalid argument. Use 'dev' for development or 'prod' for production."
  exit 1
fi

cd ./glsl2hlsl-wasm
rm -rf www pkg
wasm-pack build --target web $MODE $ARGS

cd ./ts
pnpm install
pnpm exec tsc
pnpm exec webpack --mode $WEBPACK_MODE

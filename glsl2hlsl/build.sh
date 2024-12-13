#!/usr/bin/env bash

if [ "$1" = "dev" ] || [[ -n "$GITHUB_ACTIONS" ]]; then
  ARGS="--dev --features console_error_panic_hook"
  WEBPACK_MODE="development"
elif [ "$1" = "prod" ]; then
  ARGS="--release"
  WEBPACK_MODE="production"
else
  echo "Invalid argument. Use 'dev' for development or 'prod' for production."
  exit 1
fi

cd ./glsl2hlsl-wasm
rm -rf www pkg
wasm-pack build --target web $ARGS

cd ./ts
pnpm install
pnpm exec tsc
pnpm exec webpack --mode $WEBPACK_MODE

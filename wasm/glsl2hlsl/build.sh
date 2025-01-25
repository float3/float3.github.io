#!/usr/bin/env bash

set -e

if [ "$1" = "prod" ] || [ "$GITHUB_ACTIONS" == "true" ]; then
  ARGS="--release"
  WEBPACK_MODE="production"
elif [ "$1" = "dev" ]; then
  ARGS="--dev --features console_error_panic_hook"
  WEBPACK_MODE="development"
  echo "::warning::Building in development mode."
else
  echo "Invalid argument. Use 'dev' for development or 'prod' for production."
  exit 1
fi

cd ./glsl2hlsl-wasm
rm -rf pkg
wasm-pack build --target bundler $ARGS

cd ./ts
pnpm install
pnpm exec tsc
pnpm exec webpack --mode $WEBPACK_MODE

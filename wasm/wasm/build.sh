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

rm -rf pkg
export RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
wasm-pack build --target bundler $ARGS
export RUSTFLAGS=''

cd ../../ts
pnpm install
pnpm exec tsc
pnpm exec webpack --mode $WEBPACK_MODE

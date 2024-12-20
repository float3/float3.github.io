#!/usr/bin/env bash

if [ "$1" = "prod" ] || [ "$GITHUB_ACTIONS" == "true" ]; then
  ARGS="--release"
  WEBPACK_MODE="production"
elif [ "$1" = "dev" ]; then
  ARGS="--dev --features console_error_panic_hook"
  WEBPACK_MODE="development"
  echo "::warning::Building in development mode."
else
  echo "Invalid argument. Use 'dev' for development or 'prod' for production."                                                    exit 1
fi

rm -rf pkg
wasm-pack build --target bundler $ARGS

cd ./ts
pnpm install
pnpm exec tsc
pnpm exec webpack --mode $WEBPACK_MODE

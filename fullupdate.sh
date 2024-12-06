#!/usr/bin/env bash

git pull
pnpx npm-upgrade
pnpm update
cd ts
pnpx npm-upgrade
pnpm update
cd ../tuningplayground/ts
pnpx npm-upgrade
pnpm update
cd ..
sh lint.sh

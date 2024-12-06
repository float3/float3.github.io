#!/usr/bin/env bash
cd ./ts
pnpm install
pnpx tsc
cd ..
sh ./scripts/collect_links.sh
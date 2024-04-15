#!/usr/bin/env bash
cd ./ts
npm install
npx tsc
cd ..
sh ./scripts/collect_links.sh

#!/bin/sh
cd ./programs/tuning_systems-wasm
wasm-pack build
cd ../..
npm install
tsc || true
sh ./scripts/collect_links.sh
zola serve

#!/bin/sh
cd ./programs/tuning_systems-wasm
wasm-pack build
cd ../..
npm install
tsc
sh ./scripts/collect_links.sh
zola build
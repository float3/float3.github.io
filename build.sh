
#!/bin/sh
cd ./programs/tuning_systems-wasm
wasm-pack build
cd ../..
mv ./programs/tuning_systems-wasm/pkg/* ./ts/
npm install
tsc || true
sed -i '2,3d' ./static/playground.js
sh ./scripts/collect_links.sh
zola build
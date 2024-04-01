
#!/bin/sh
cd ts
npm install
npx tsc
sh ./scripts/collect_links.sh
zola serve
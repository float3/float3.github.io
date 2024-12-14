#!/usr/bin/env bash

git config --global user.email "github-actions[bot]@users.noreply.github.com"
git config --global user.name "github-actions[bot]"
git add -A
git pull
git diff --staged --quiet || git commit -m "generate"
git push || echo "No changes to commit"

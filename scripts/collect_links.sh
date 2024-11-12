#!/bin/sh
cat ./content/notes/talks.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/static/plaintext/talks.txt
cat ./content/notes/blogs.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./content/plaintext/blogs.txt
cat ./content/notes/graphics-resources.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" | sort -u >  ./content/plaintext/graphics-resources.txt

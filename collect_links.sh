#!/bin/sh
cat ./content/notes/talks.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./static/talks.txt
cat ./content/notes/blogs.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" >  ./static/blogs.txt
cat ./content/notes/graphics-resources.md | grep -Eo "(http|https)://[a-zA-Z0-9./?=_%:-]*" | sort -u >  ./static/graphics-resources.txt

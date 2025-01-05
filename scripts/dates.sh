#!/usr/bin/env bash

# This script sets "date" to the file's first commit date (creation)
# and "updated" to the file's last commit date (ignoring commits
# whose commit message contains the word "generate").

if [ -n "$GITHUB_ACTIONS" ]; then
  git fetch --unshallow
fi

for file in $(find . -type f -name "*.md"); do
  created_date="$(git log --diff-filter=A --follow --format=%aI -1 -- "$file" | cut -dT -f1)"
  # Skip any commits that have "generate" in the commit message
  updated_date="$(git log --invert-grep --grep='generate' -1 --format=%aI -- "$file" | cut -dT -f1)"

  # Remove existing date/updated lines if present
  sed -i '/^date:/d' "$file"
  sed -i '/^updated:/d' "$file"

  # Insert date/updated after the 'title:' line if it exists,
  # otherwise just prepend to the start of the file if there's no front matter.
  if grep -q '^title:' "$file"; then
    sed -i "/^title:.*/a date: $created_date\nupdated: $updated_date" "$file"
  else
    sed -i "1s|^|date: $created_date\nupdated: $updated_date\n\n|" "$file"
  fi
done

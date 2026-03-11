#!/usr/bin/env bash

fd . "$1" -t f | while read -r file; do
  echo "===== $file ====="
  cat "$file"
  echo
done | xclip -selection clipboard

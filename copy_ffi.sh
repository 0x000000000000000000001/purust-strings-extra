#!/bin/bash
echo "Copying FFI files to .spago..."
find ../purust-*/src -name "*.rs" -type f | while read f; do
  # e.g. f = ../purust-strings/src/Data/String/Regex.rs
  # We want to find the corresponding .purs in .spago and copy the .rs there
  relpath=$(echo "$f" | sed 's|^../purust-[^/]*/src/||')
  # e.g. relpath = Data/String/Regex.rs
  purs_path=$(find .spago -path "*/src/${relpath%.rs}.purs" 2>/dev/null | head -n 1)
  if [ -n "$purs_path" ]; then
    rs_dest="${purs_path%.purs}.rs"
    cp "$f" "$rs_dest"
    echo "Copied $f to $rs_dest"
  fi
done

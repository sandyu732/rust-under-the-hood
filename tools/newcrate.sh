#!/usr/bin/env bash
# newcrate.sh <dir> <name> [dep lines...]   -> writes Cargo.toml
dir=$1; name=$2; shift 2
mkdir -p "$dir/src" "$dir/tests"
{
  echo "[package]"
  echo "name = \"$name\""
  echo "version.workspace = true"
  echo "edition.workspace = true"
  echo "publish.workspace = true"
  echo
  echo "[dependencies]"
  for d in "$@"; do echo "$d"; done
} > "$dir/Cargo.toml"

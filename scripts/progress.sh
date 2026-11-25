#!/usr/bin/env bash
# Run every lab's assessment tests and print a scoreboard.
#   ./scripts/progress.sh            # all labs
#   ./scripts/progress.sh m02        # only labs whose path contains "m02"
set -uo pipefail
cd "$(dirname "$0")/.."

filter="${1:-}"
pass=0; fail=0
printf "%-34s %s\n" "LAB" "RESULT"
printf "%-34s %s\n" "---" "------"
for manifest in labs/*/lab-*/Cargo.toml capstones/*/Cargo.toml; do
  [[ -f "$manifest" ]] || continue
  [[ -n "$filter" && "$manifest" != *"$filter"* ]] && continue
  name=$(grep -m1 '^name' "$manifest" | sed -E 's/name *= *"(.*)"/\1/')
  if cargo test -q -p "$name" >/dev/null 2>&1; then
    printf "%-34s \033[32mPASS\033[0m\n" "$name"; pass=$((pass + 1))
  else
    printf "%-34s \033[31mTODO\033[0m\n" "$name"; fail=$((fail + 1))
  fi
done
echo
echo "Passed $pass / $((pass + fail))"

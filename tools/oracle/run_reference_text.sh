#!/bin/sh
# Text-mode output of the reference for phase 1: every project it rejects in phase 1, plus
# every module-level case (accepted ones included, for the progress and summary lines).
for list in projects project_cases phase1_cases; do
  while IFS="$(printf '\t')" read -r id rc manifest; do
    if [ "$rc" != 0 ] || [ "$list" = phase1_cases ]; then
      dir=$(dirname "/w/$manifest")
      printf '### %s\n' "$manifest"
      (cd "$dir" && /w/reference/ultraviolet/uvc build Ultraviolet.toml --phase1-only --color never --target-profile x86_64-sysv --no-crash-report 2>&1 | sed 's/ in [0-9.]*m\?s$/ in <t>/')
      rm -rf "$dir/Build"
    fi
  done < "/w/tests/golden/$list/phase1.tsv"
done

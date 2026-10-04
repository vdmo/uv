#!/bin/sh
cd /tmp
n=0
while IFS= read -r line; do
  n=$((n + 1))
  out=$(/w/reference/ultraviolet/uvc $line 2>/tmp/err)
  rc=$?
  printf '### %s | %s\nrc=%s\n--stdout--\n%s\n--stderr--\n%s\n' "$n" "$line" "$rc" "$out" "$(cat /tmp/err)"
done < /w/tests/golden/cli_cases.txt

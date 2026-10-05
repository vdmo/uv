#!/bin/sh
# Runs inside the uv-oracle container with the workspace mounted at /w.
set -eu
B=/w/ultraviolet/Bootstrap
S=$B/Ultraviolet/src
OUT=/w/reference/oracle
mkdir -p "$OUT/obj"
# The analysis and project sources are the ones the reference build lists; the tree also
# holds files that are not part of it.
ANALYSIS=$(grep -o '04_analysis/[A-Za-z0-9_/]*\.cpp' "$S/CMakeLists.txt" | sort -u | sed "s|^|$S/|")
PROJECT=$(grep -o '01_project/[A-Za-z0-9_/]*\.cpp' "$S/CMakeLists.txt" | sort -u | sed "s|^|$S/|")
SRCS="$(ls $S/00_core/*.cpp) $S/00_core/host/services.cpp $S/00_core/host/linux_host.cpp $S/00_core/host/crash_debug.cpp $S/00_core/host/crash_debug_linux.cpp $(find $S/02_source -name '*.cpp' ! -name parse_modules.cpp | sort) $(ls $S/03_comptime/*.cpp) $ANALYSIS $PROJECT /w/tools/oracle/oracle_main.cpp"
OBJS=""
for src in $SRCS; do
  obj="$OUT/obj/$(echo "$src" | sed 's|/|_|g').o"
  OBJS="$OBJS $obj"
  if [ ! -f "$obj" ] || [ "$src" -nt "$obj" ] || [ /w/tools/oracle/ast_dump_generated.inc -nt "$obj" -a "$src" = /w/tools/oracle/oracle_main.cpp ]; then
    while [ "$(pgrep -c cc1plus || true)" -ge 4 ]; do sleep 0.3; done
    g++ -std=c++20 -O1 -w -c "$src" -o "$obj" \
      -I"$B/Ultraviolet/include" -I"$B/Ultraviolet/src" -I/w/tools/oracle -I"$B/extern/icu/linux/include" -I"$B/extern/tomlplusplus/include" &
  fi
done
wait
g++ $OBJS -o "$OUT/uv-oracle" -L"$B/extern/icu/linux/lib" \
  -l:libicui18n.so.72 -l:libicuuc.so.72 -l:libicudata.so.72 -lpthread -ldl \
  -Wl,-rpath,"$B/extern/icu/linux/lib"
echo built "$OUT/uv-oracle"

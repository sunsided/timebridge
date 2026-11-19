#!/bin/sh
# Fail on `// ----` style separator comments in Rust files (split the file into modules instead).
# Shared by `task lint:separators` and the CI `style` job.
grep -rnE --include='*.rs' --exclude-dir=target --exclude-dir=.git \
  '^[[:space:]]*//+[[:space:]]*[-=*#~_/]{4,}' .
status=$?

# grep exits 1 for "no match"; anything above that is a failed scan, not a clean one.
case $status in
  0) echo "separator comments found: split the file into modules instead"; exit 1 ;;
  1) exit 0 ;;
  *) exit "$status" ;;
esac

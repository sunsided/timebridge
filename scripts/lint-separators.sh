#!/bin/sh
# Fail on `// ----` style separator comments in Rust files (split the file into modules instead).
# Shared by `task lint:separators` and the CI `style` job.
if grep -rnE --include='*.rs' --exclude-dir=target --exclude-dir=.git \
  '^[[:space:]]*//+[[:space:]]*[-=*#~_/]{4,}' .; then
  echo "separator comments found: split the file into modules instead"
  exit 1
fi

#!/bin/sh
# Fail on relative links or images in README files (crates.io needs absolute URLs).
# Shared by `task lint:readme` and the CI `style` job.
# docs/review/README.md is a GitHub-only index with generated relative links.
if grep -rnoE --include='README.md' --exclude-dir=target --exclude-dir=.git --exclude-dir=review \
  '(\]\(|src=")[^)"]*' . \
  | grep -vE ':(\]\(|src=")(https?://|#|mailto:)'; then
  echo "use absolute URLs for these links"
  exit 1
fi

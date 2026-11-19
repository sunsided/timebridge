---
title: CI, Taskfile and hooks
applyTo: ".github/workflows/**,Taskfile.dist.yaml,.pre-commit-config.yaml,scripts/**"
summary: Keep task ci, ci.yml and hooks in sync; pinned, least-privilege workflows.
---

# CI, Taskfile and hooks stay in sync

- The `ci` task in `Taskfile.dist.yaml` mirrors the jobs in `.github/workflows/ci.yml`
  (`branch`, `fmt`, `clippy`, `style`, `doc`, `test`, `deny`, `msrv`). `coverage` and `semver`
  stay out of `task ci` on purpose (extra tools, and `semver` needs the published baseline);
  they have their own `task coverage` and `task semver`. A change that adds, removes or
  renames a CI job, or changes its flags, must make the same change to the matching task, and
  the other way round.
- Shell checks used by both CI and the Taskfile live in `scripts/`. Neither side inlines them.
- The `test` job runs on Linux, macOS and Windows. Keep Windows: `SystemTime` has 100 ns
  resolution there, and `timebridge`'s `SystemTime` conversion relies on that being tested.
- Local hooks in `.pre-commit-config.yaml` call Taskfile tasks. They must not duplicate
  commands.
- Every `uses:` is pinned to a full 40-character commit SHA, with a `# vX.Y.Z` comment. Reject
  tag or branch refs.
- Workflows set `permissions: {}` at the top, and each job sets the least privilege it
  needs. Every `actions/checkout` sets `persist-credentials: false`.
- No `pull_request_target`. No `${{ github.event.* }}` or other untrusted expressions
  directly in `run:`. Pass them through `env:`.
- Do not rename `publish-crates-io.yml` or its `crates-io` environment. crates.io trusted
  publishing for all five crates is bound to both. The `crates-io` job `needs:` `verify`.
- `scripts/check-branch.sh` is the single definition of allowed branch names (GitHub Flow).
  It is used by the Taskfile, the pre-push hook and the CI `branch` job.
- New jobs that should gate merges must also be added to the default-branch ruleset. Mention
  that in the PR.

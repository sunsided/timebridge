---
title: Documentation
applyTo: "**/*.md,docs/**"
summary: Absolute URLs, valid and current mermaid, doctested README examples.
---

# Documentation

- README files are rendered on crates.io and docs.rs. Every link to a repo file and every
  image uses an absolute URL (`https://github.com/...`, `https://raw.githubusercontent.com/...`).
  Only in-page anchors may be relative.
- Mermaid diagrams (```` ```mermaid ````) must be valid (`task lint:mermaid` renders them)
  and must match the code. Flag diagrams that the change makes stale. Keep each diagram under
  about 15 nodes.
- Code blocks are tagged with their language. Untagged or `rust` blocks in included Markdown
  run as doctests, so they must compile. Use `rust,no_run` or `rust,ignore` only with a
  stated reason.
- Keep badges, the License section and the MSRV statement consistent with `Cargo.toml`.
- Each crate's `README.md` is doctested through the `readme_doctests` module in its
  `lib.rs`. Examples there must compile and pass.

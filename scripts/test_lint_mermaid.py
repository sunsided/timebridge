#!/usr/bin/env python3
"""Tests for the block detection in lint-mermaid.py (run: python3 -m unittest discover -s scripts)."""

import importlib.util
import tempfile
import unittest
from pathlib import Path

SPEC = importlib.util.spec_from_file_location("lint_mermaid", Path(__file__).with_name("lint-mermaid.py"))
lint_mermaid = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(lint_mermaid)


def blocks(name, text):
    """Return the diagrams() output for a temporary file with this name and content."""
    with tempfile.TemporaryDirectory() as workdir:
        path = Path(workdir) / name
        path.write_text(text, encoding="utf-8")
        return list(lint_mermaid.diagrams(path))


class DiagramsTest(unittest.TestCase):
    """Block detection in Markdown and Rust doc comments."""

    def test_closed_markdown_block(self):
        """A fenced block is yielded with its start line and marked closed."""
        self.assertEqual(blocks("a.md", "# x\n```mermaid\nflowchart TD\n```\n"), [(2, "flowchart TD\n", True)])

    def test_unclosed_markdown_block(self):
        """A block still open at end of file is yielded as unclosed, even if its body is valid."""
        self.assertEqual(blocks("a.md", "```mermaid\nflowchart TD\nA --> B\n"), [(1, "flowchart TD\nA --> B\n", False)])

    def test_rust_doc_block_ends_at_code(self):
        """In Rust, the first non-doc line ends the block."""
        self.assertEqual(blocks("a.rs", "//! ```mermaid\n//! graph LR\nmod x;\n"), [(1, "graph LR\n", True)])


if __name__ == "__main__":
    unittest.main()

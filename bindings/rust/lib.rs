//! Sass (indented syntax) grammar for tree-sitter.
//!
//! Vendored from <https://github.com/bajrangCoder/tree-sitter-sass>.

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_sass() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_sass) };

/// The content of the [`node-types.json`] file for this grammar.
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// Upstream syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

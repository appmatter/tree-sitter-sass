//! Sass (indented syntax) grammar for tree-sitter.
//!
//! Fork of <https://github.com/bajrangCoder/tree-sitter-sass> with Rust bindings.

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

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::{Parser, Query};

    fn parse(src: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("load sass language");
        parser.parse(src, None).expect("parse sass")
    }

    #[test]
    fn loads_language() {
        let mut parser = Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Sass parser");
    }

    #[test]
    fn parses_simple_indented_example() {
        let src = include_str!("../../examples/simple.sass");
        let tree = parse(src);
        assert!(
            !tree.root_node().has_error(),
            "parse errors in simple.sass: {}",
            tree.root_node().to_sexp()
        );
    }

    #[test]
    fn parses_variables_and_mixin_shorthand() {
        let src = concat!(
            "$accent: #4ec9b0\n",
            "=card($radius: 4px)\n",
            "  border-radius: $radius\n",
            ".title\n",
            "  color: $accent\n",
            "  +card(8px)\n",
        );
        let tree = parse(src);
        assert!(
            !tree.root_node().has_error(),
            "parse errors: {}",
            tree.root_node().to_sexp()
        );
    }

    #[test]
    fn highlights_query_compiles() {
        let language = LANGUAGE.into();
        Query::new(&language, HIGHLIGHTS_QUERY).expect("highlights query should compile");
    }
}

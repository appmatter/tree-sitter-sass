# tree-sitter-sass

A [Tree-sitter](https://tree-sitter.github.io/) grammar for **Sass indented syntax** (`.sass` files).

## License

MIT

## Rust

```toml
tree-sitter-sass = { git = "https://github.com/appmatter/tree-sitter-sass" }
```

This fork adds a thin Rust binding (`bindings/rust`) on top of the upstream grammar so Cargo can depend on it.


## Tests

```bash
cargo test
```

# collections2

> Because `collections` is a reserved name on crates.io ✨

A crate for generic collections, such as `Map`, `Set`, and `List`, including mutable variants. Includes an `Iterable` and `IterableMut` trait for generic iterators.

Supports `no_std`, just add `default-features = false` to your cargo dependency. Supports opt-in alloc with the `alloc` feature.

Requires Rust 1.65+ due to usage of generic associated types (GATs).

## License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
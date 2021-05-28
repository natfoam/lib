# lib

Rust libraries:

- [fixed-array](fixed-array) is a trait for fixed-size arrays.
- [uints](uints) are traits and extensions for different unsigned integer types.
- [list-fn](list-fn) is a lazy-list as an alternative to the standard Rust iterator.
- [bit-list](bit-list) is an LSB-first list of bits.
- [sha2-compress](sha2-compress) is an SHA2 compress function.
- [u144](u144) is an unsigned integer 144 bit. For example, to store 137 bit blocks 🙄.
- [publish-ws](publish-ws) is a tool for publishing all workspace packages.

## Conventions

- a `Fn` suffix is used for traits. For example, `ListFn`.
- a name of an extension function is used for its trait. For example
  ```rust
  trait Fold: ListFn {
      fn fold(self) -> Self::End { ... }
  }
  impl<T: ListFn> Fold for T {}
  ```

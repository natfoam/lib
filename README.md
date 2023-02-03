# lib

[![Netlify Status](https://api.netlify.com/api/v1/badges/fb9af04c-2f5c-43ed-8d46-88b7e298a4e9/deploy-status)](https://app.netlify.com/sites/natfoam-lib/deploys)

Documentation:
- on [Cloudflare](https://natfoam-lib.pages.dev),
- on [Netlify](https://natfoam-lib.netlify.app)

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
  An alternative is to use `Sugar` suffix:
  ```rust
  trait FoldSugar: ListFn {
      fn fold(self) -> Self::End { ... }
  }
  impl<T: ListFn> FoldSugar for T {}
  ```

## Rust Wish List

- `const fn` in traits. For example
  ```rust
  trait X {
      const fn x() -> X;
  }
  ```
- default types in traits
  ```rust
  trait X {
      type M = u8
  }
  ```
- `impl Type` in traits almost like `dyn`.
  ```rust
  trait X {
      type M = impl R;
  }
  ```
- defining arrays in traits using parameters
  ```rust
  trait X<const N: usize> {
      fn x() -> [u8; N];
  }
  ```
- [Generators](https://doc.rust-lang.org/std/ops/trait.Generator.html). See [genawaiter](https://crates.io/crates/genawaiter) as an example.

# lib

Rust libraries

## Conventions

- a `Fn` suffix is used for traits. For example, `ListFn`.
- a name of an extension function is used for its trait. For example
  ```rust
  trait Fold: ListFn {
      fn fold(self) -> Self::End { ... }
  }
  ```

# list-fn

A generic lazy list. It's an immutable iterator with continuations.

The lazy list can be used as an immutable alternative to the standard Rust iterator.

Any `List<End = Self>` is an iterator. Any `&mut Iterator` is a `List<End = Self>`.

## Fundamental Operations

- `FlatScan`. An order of items is important.
  - `Scan`
  - `Fold`
- `FlatMap`. An order of items is not important.
  - `Map`
  - `Filter`
  - `Flatten`

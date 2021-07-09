# list-fn

A generic lazy list. It's an immutable iterator with continuations.

The lazy list can be used as an immutable alternative to the standard Rust iterator.

Any `List<End = Self>` is an iterator. Any `&mut Iterator` is a `List<End = Self>`.

See also [Generators](https://doc.rust-lang.org/std/ops/trait.Generator.html).

## Fundamental Operations

- `FlatScan`. An order of items is important.
  - `FilterScan`
  - `Scan`
  - `Fold`
- `FlatMap`. An order of items is not important.
  - `Map`
  - `Filter`
  - `Flatten`

## Evolution

### 1. Iterator

```rust
trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;
}

trait IteratorReturn: Iterator {
  type Return;
  fn return(&mut self) -> Return;
}
```

### 2. Generator

```rust
trait Generator {
  type Yield;
  type Return;
  fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorState<Y, R> {
  Yielded(Y),
  Complete(R),
}
```

### 3. List

```rust
trait List {
  type Yield;
  type Return;
  fn resume(self) -> ListState<Self>;
}

enum ListState<L: List> {
  Yielded(L::Yield, L),
  Return(L::Return),
}
```

## FlatScan

### 3. List

```rust
trait FlatScan {
  type InputYield;
  type InputReturn;
  type OutputYield;
  type OutputReturn;
  fn map_yield(self, input_yield: Self::InputYield) -> impl List<Yield = Self::OutputYield, Return = Self>;
  fn map_return(self, input_return: Self::InputReturn) -> impl List<Yield = Self::OutputYield, Return = OutputReturn>;
}
```

### 2. Generator

```rust
trait FlatScan {
  type InputYield;
  type InputReturn;
  type OutputYield;
  type OutputReturn;
  fn map_yield(&mut self, input_yield: Self::InputYield) -> impl Iterator<Item = Self::OutputYield>;
  fn map_return(&mut self, input_return: Self::InputReturn) -> 
    impl Generator<Item = Self::OutputYield, Return = Self::OutputReturn>;
}
```

### 1. Iterator

```rust
trait FlatScan {
  type InputYield;
  type InputReturn;
  type OutputYield;
  type OutputReturn;
  fn map_yield(&mut self, input_yield: Self::InputYield) -> impl Iterator<Item = Self::OutputYield>;
  fn map_return(&mut self, input_return: Self::InputReturn) -> 
    impl Iterator<Item = Self::OutputYield> + IteratorReturn<Return = Self::OutputReturn>;
}
```
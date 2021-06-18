use super::*;

pub struct IteratorGenerator<I>(I);

/// Any `&mut Iterator` is a `ListFn<End = Self>`.
impl<I: Iterator> Generator for IteratorGenerator<I> {
    type Yield = I::Item;
    type Return = ();
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        match self.0.next() {
            Some(i) => GeneratorState::Yielded(i),
            None => GeneratorState::Completed(()),
        }
    }
}

pub struct GeneratorIterator<G>(G);

impl<G: Generator<Return = ()>> Iterator for GeneratorIterator<G> {
    type Item = G::Yield;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.resume() {
            GeneratorState::Yielded(y) => Some(y),
            GeneratorState::Completed(()) => None,
        }
    }
}

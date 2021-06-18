use super::*;

pub trait FlatMap {
    type Input;
    type OutputIterator: Iterator;
    fn call(self, input: Self::Input) -> Self::OutputIterator;
}

//

pub struct FlatMapGenerator<I, F: FlatMap> {
    pub input: I,
    pub flat_map: F,
    pub list: Option<F::OutputIterator>,
}

impl<I: Generator, F: FlatMap<Input = I::Yield> + Copy> Generator for FlatMapGenerator<&mut I, F> {
    type Yield = <F::OutputIterator as Iterator>::Item;
    type Return = I::Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        loop {
            match &mut self.list {
                Some(list) => match list.next() {
                    Some(item) => return GeneratorState::Yielded(item),
                    None => self.list = None,
                },
                None => match self.input.resume() {
                    GeneratorState::Yielded(y) => self.list = Some(self.flat_map.call(y)),
                    GeneratorState::Completed(c) => return GeneratorState::Completed(c),
                },
            }
        }
    }
}

pub trait FlatMapSugar: Generator + Sized {
    fn flat_map<F: FlatMap<Input = Self::Yield>>(self, f: F) -> FlatMapGenerator<Self, F>;
}

impl<G: Generator> FlatMapSugar for G {
    fn flat_map<F: FlatMap<Input = Self::Yield>>(self, flat_map: F) -> FlatMapGenerator<Self, F> {
        FlatMapGenerator {
            input: self,
            flat_map,
            list: None,
        }
    }
}

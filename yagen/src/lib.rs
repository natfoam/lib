mod iter;
mod flat_map;

use iter::*;
use flat_map::*;

pub trait Generator {
    type Yield;
    type Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Completed(R),
}

mod compose;
mod flat_map;
mod flat_scan;
mod iter;

pub use compose::*;
pub use flat_map::*;
pub use flat_scan::*;
pub use iter::*;

pub trait Generator {
    type Yield;
    type Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

pub enum GeneratorState<Y, R> {
    Yielded(Y),
    Completed(R),
}

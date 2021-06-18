use std::mem::replace;

use super::*;

pub trait FlatScan {
    type Input;
    type OutputGenerator: Generator;
    fn call(
        self,
        input: Self::Input,
        prior: Option<<Self::OutputGenerator as Generator>::Return>,
    ) -> Self::OutputGenerator;
}

//

pub struct FlatScanGenerator<I, F: FlatScan> {
    input: I,
    flat_scan: F,
    list: Option<F::OutputGenerator>,
}

impl<I: Generator, F: FlatScan<Input = I::Yield> + Copy> Generator for FlatScanGenerator<I, F> {
    type Yield = <F::OutputGenerator as Generator>::Yield;
    type Return = (Option<<F::OutputGenerator as Generator>::Return>, I::Return);
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        let mut r = None;
        loop {
            match &mut self.list {
                Some(list) => match list.resume() {
                    GeneratorState::Yielded(y) => return GeneratorState::Yielded(y),
                    GeneratorState::Completed(c) => {
                        self.list = None;
                        r = Some(c);
                    }
                },
                None => match self.input.resume() {
                    GeneratorState::Yielded(y) => {
                        self.list = Some(self.flat_scan.call(y, replace(&mut r, None)));
                    }
                    GeneratorState::Completed(c) => return GeneratorState::Completed((r, c)),
                },
            }
        }
    }
}

pub trait FlatScanSugar: Generator + Sized {
    fn flat_scan<F: FlatScan<Input = Self::Yield>>(
        self,
        flat_scan: F,
    ) -> FlatScanGenerator<Self, F>;
}

impl<G: Generator> FlatScanSugar for G {
    fn flat_scan<F: FlatScan<Input = Self::Yield>>(
        self,
        flat_scan: F,
    ) -> FlatScanGenerator<Self, F> {
        FlatScanGenerator {
            input: self,
            flat_scan,
            list: None,
        }
    }
}

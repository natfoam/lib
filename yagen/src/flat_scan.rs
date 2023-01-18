// use std::mem::replace;

// use super::*;

pub trait FlatScan {
    type YieldInput;
    type ReturnInput;
    type YieldOutputIterator: Iterator;
    type ReturnOutput;
    fn map_yield(&mut self, yield_input: Self::YieldInput) -> Self::YieldOutputIterator;
    fn map_return(&mut self, return_input: Self::ReturnInput) -> Self::ReturnOutput;
}

/*
pub trait FlatScan {
    type YieldInput;
    type ReturnInput;
    type OutputGenerator: Generator;
    fn map_yield(
        self,
        prior: Option<<Self::OutputGenerator as Generator>::Return>,
        input: Self::YieldInput,
    ) -> Self::OutputGenerator;
    fn map_return(
        self,
        prior: Option<<Self::OutputGenerator as Generator>::Return>,
        input: Self::ReturnInput,
    ) -> Self::OutputGenerator;
}

//

pub struct FlatScanGenerator<I, F: FlatScan> {
    input: I,
    flat_scan: F,
    ret: Option<<F::OutputGenerator as Generator>::Return>,
    list: Option<F::OutputGenerator>,
}

impl<I: Generator, F: FlatScan<Input = I::Yield> + Copy> Generator for FlatScanGenerator<I, F> {
    type Yield = <F::OutputGenerator as Generator>::Yield;
    type Return = (Option<<F::OutputGenerator as Generator>::Return>, I::Return);
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
        loop {
            match &mut self.list {
                Some(list) => match list.resume() {
                    GeneratorState::Yielded(y) => return GeneratorState::Yielded(y),
                    GeneratorState::Completed(c) => {
                        self.list = None;
                        self.ret = Some(c);
                    }
                },
                None => {
                    let ret = replace(&mut self.ret, None);
                    match self.input.resume() {
                        GeneratorState::Yielded(y) => self.list = Some(self.flat_scan.map_input(ret, y)),
                        GeneratorState::Completed(c) => return GeneratorState::Completed((ret, c)),
                    }
                }
            }
        }
    }
}

pub trait FlatScanSugar: Generator + Sized {
    fn flat_scan<F: FlatScan<Input = Self::Yield>>(
        self,
        flat_scan: F,
        ret: Option<<F::OutputGenerator as Generator>::Return>,
    ) -> FlatScanGenerator<Self, F>;
}

impl<G: Generator> FlatScanSugar for G {
    fn flat_scan<F: FlatScan<Input = Self::Yield>>(
        self,
        flat_scan: F,
        ret: Option<<F::OutputGenerator as Generator>::Return>,
    ) -> FlatScanGenerator<Self, F> {
        FlatScanGenerator {
            input: self,
            flat_scan,
            ret,
            list: None,
        }
    }
}
*/

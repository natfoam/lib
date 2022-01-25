// use super::*;

pub struct Compose<F0, F1>(F0, F1);

/*
impl<F0: FlatScan, F1: FlatScan<Input = <F0::OutputGenerator as Generator>::Yield> + Copy> FlatScan for Compose<F0, F1> {
    type Input = F0::Input;
    type OutputGenerator = FlatScanGenerator<F0::OutputGenerator, F1>;
    fn map_input(
        self,
        ret: Option<<Self::OutputGenerator as Generator>::Return>,
        input: Self::Input,
    ) -> Self::OutputGenerator {
        let r = self.0.map_input(ret.map(|v| v.1), input);
        r
    }
}

pub trait ComposeSugar: FlatScan + Sized {
    fn compose<F: FlatScan>(self, f: F) -> Compose<Self, F>;
}

impl<S: FlatScan> ComposeSugar for S {
    fn compose<F: FlatScan>(self, f: F) -> Compose<Self, F> {
        Compose(self, f)
    }
}
*/
pub trait ResultFn {
    type Result;
    fn result(self) -> Self::Result;
}

impl ResultFn for () {
    type Result = ();
    fn result(self) {}
}

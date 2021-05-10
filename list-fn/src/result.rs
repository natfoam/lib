pub trait ResultFn {
    type Result;
    fn result(self) -> Self::Result;
}

pub trait ResultFn {
    type Result;
    fn result(self) -> Self::Result;
}

impl ResultFn for () {
    type Result = ();
    fn result(self) {}
}

pub struct Id<T>(T);

impl<T> ResultFn for Id<T> {
    type Result = T;
    fn result(self) -> T {
        self.0
    }
}

impl<T> Id<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}

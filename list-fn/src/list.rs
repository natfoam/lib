pub enum List<L: ListFn> {
    Some(L::Item, L),
    End(L::End),
}

/// A function which returns a list.
pub trait ListFn: Sized {
    type Item;
    type End;
    fn list(self) -> List<Self>;
}

pub trait Stack: IntoIterator
where
    Self::IntoIter: DoubleEndedIterator,
{
    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T> Stack for Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn push(&mut self, value: Self::Item) {
        self.push(value)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

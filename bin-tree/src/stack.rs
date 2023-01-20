pub trait Stack: IntoIterator {
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

pub struct DebugStack<T> {
    vec: Vec<T>,
    usage: usize,
}

impl<T> DebugStack<T> {
    pub fn usage(&self) -> usize {
        self.usage
    }
}

impl<T> Stack for DebugStack<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            usage: 0,
        }
    }
    fn push(&mut self, value: T) {
        self.vec.push(value);
        self.usage = self.usage.max(self.vec.len());
    }
    fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}

impl<T> IntoIterator for DebugStack<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

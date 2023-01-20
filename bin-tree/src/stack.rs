pub struct Stack<T> {
    vec: Vec<T>,
    usage: usize,
}

impl<T> Stack<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            usage: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        self.vec.push(value);
        self.usage = self.usage.max(self.vec.len());
    }
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

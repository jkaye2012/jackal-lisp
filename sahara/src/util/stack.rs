#[derive(Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> T {
        self.items.pop().expect("Attempted to pop empty stack")
    }

    pub fn peek(&self) -> &T {
        self.items.last().expect("Attempted to peek empty stack")
    }

    pub fn peek_mut(&mut self) -> &mut T {
        self.items
            .last_mut()
            .expect("Attempted to peek empty stack")
    }
}

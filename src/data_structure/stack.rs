pub struct Stack<T: Ord>{
    elements: Vec<T>
}

impl<T: Ord> Stack<T>{
    pub fn new() -> Self {
        Stack { elements: Vec::new()}
    }

    pub fn push(&mut self, item: T) {
        self.elements.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn stack_test() {
        let mut stack: Stack<i32> = Stack::new();

        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }
}
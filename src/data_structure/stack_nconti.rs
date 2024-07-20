struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct StackNonConti<T>{
    top: Option<Box<Node<T>>>
}

impl<T> StackNonConti<T>{
    pub fn new() -> Self {
        StackNonConti{ top: None }
    }

    pub fn push(&mut self, data: T){
        let new_node = Box::new(Node {
            data,
            next: self.top.take(),
        });

        self.top = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T>{
        self.top.take().map(|mut top_node| {
            self.top = top_node.next.take();
            top_node.data
        })
    }

    pub fn peek(&self) -> Option<&T>{
        match &self.top {
            None => None,
            Some(node) => Some(&node.data)
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.top {
            None => true,
            Some(_) => false,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn stack_nconti_push(){
        let mut stack = StackNonConti::new();

        assert!(stack.top.is_none());

        stack.push(1);
        stack.push(2);

        assert_eq!(stack.top.as_ref().map(|node| node.data), Some(2));

        stack.push(3);

        assert_eq!(stack.top.as_ref().map(|node| node.data), Some(3));
    }

    #[test]
    fn stack_nconti_pop(){
        let mut stack = StackNonConti::<i32>::new();
        assert_eq!(stack.pop(), None);
        
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.pop(), Some(2));

    }

    #[test]
    fn stack_nconti_is_empty(){
        let mut stack = StackNonConti::<i32>::new();
        assert_eq!(stack.is_empty(), true);
        
        stack.push(1);
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn stack_nconti_peek(){
        let mut stack: StackNonConti<i32> = StackNonConti::new();
        assert!(stack.peek().is_none());

        stack.push(1);
        assert_eq!(*stack.peek().unwrap(), 1);

        stack.pop();
        assert!(stack.peek().is_none());
    }

}
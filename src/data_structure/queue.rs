pub struct Queue<T: Ord>{
    data: Vec<T>
}

impl<T: Ord> Queue<T>{
    pub fn new() -> Self {
        Queue{data: Vec::new()}
    }

    pub fn enqueue(&mut self, value: T) {
        self.data.push(value)
    }

    pub fn dequeue(&mut self) -> Option<T>{
        match &mut self.data.len(){
            0 => None,
            _ => Some(self.data.remove(0))
        }
    }

    pub fn peek(&self) -> Option<&T>{
        self.data.first()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}




#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn initialize_queue(){
        let queue = Queue::<i32>::new();

        assert!(queue.data.is_empty());
    }

    #[test]
    fn enqueue(){
        let mut queue = Queue::<i32>::new();

        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.data, [1,2]);
    }

    #[test]
    fn dequeue(){
        let mut queue = Queue::<i32>::new();
        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn peek(){
        let mut queue = Queue::<i32>::new();
        queue.enqueue(2);

        assert_eq!(queue.peek(), Some(&2));

        
        assert_eq!(queue.data, [2]);
    }

    #[test]
    fn is_empty(){
        let mut queue = Queue::<i32>::new();
        assert!(queue.is_empty());

        queue.enqueue(1);
        queue.enqueue(2);
        assert!(!(queue.is_empty()))
    }
}
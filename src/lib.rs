pub mod sorting;
pub mod searching;
pub mod data_structure;



#[cfg(test)]
mod tests{

    use crate::searching::{linear_search, binary_search};

    #[test]
    fn test_linear_search(){
        let mut array = vec![4, 2, 5, 1, 3];
        let existing_value = 1;
        let non_existing_value = 0;
        let exisiting_result = linear_search(&mut array, existing_value);
        let non_exisiting_result = linear_search(&mut array, non_existing_value);
        assert_eq!(exisiting_result, Some(3));
        assert_eq!(non_exisiting_result, None);
    }

    #[test]
    fn test_binary_search(){
        let mut array = vec![4, 2, 5, 1, 3];
        array.sort();
        let existing_value = 1;
        let non_existing_value = 0;
        let exisiting_result = binary_search(&array, existing_value);
        let non_exisiting_result = binary_search(&array, non_existing_value);
        assert_eq!(exisiting_result, Some(0));
        assert_eq!(non_exisiting_result, None);
    }

    use crate::data_structure::{LinkedList, Stack};

    #[test]
    fn linked_list_test_add() {
        let mut list = LinkedList::new(1);
        list.add(2);
        list.add(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_test_add_single() {
        let mut list = LinkedList::new(10);
        list.add(20);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_test_add_multiple() {
        let mut list = LinkedList::new(1);
        for i in 2..=5 {
            list.add(i);
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_find_method_various_scenarios() {
        let mut list = LinkedList::new(1);
        list.add(2);
        list.add(3);

        // Find head
        assert_eq!(list.find(1).unwrap().data, 1, "Failed to find the head element.");

        // Find middle
        assert_eq!(list.find(2).unwrap().data, 2, "Failed to find the middle element.");

        // Find tail
        assert_eq!(list.find(3).unwrap().data, 3, "Failed to find the tail element.");

        //Trying to find a non-existent element
        assert!(list.find(4).is_none(), "Non-existent element was supposedly found.");
    }

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
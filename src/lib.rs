pub mod sorting;
pub mod searching;
pub mod data_structure;



#[cfg(test)]
mod tests{
    use crate::sorting::{bubble_sort, selection_sort, insertion_sort, quick_sort, counting_sort, radix_sort, merge_sort};

    #[test]
    fn test_bubble_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        bubble_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        selection_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        insertion_sort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        let len = array.len();
        quick_sort(&mut array, 0, (len - 1) as isize);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort() {
        let mut array = vec![4, 2, 5, 2, 1, 1, 0, 3];
        counting_sort(&mut array);
        assert_eq!(array, vec![0, 1, 1, 2, 2, 3, 4, 5]);
    }

    #[test]
    fn test_radix_sort(){
        let mut array = vec![170, 45, 75, 90, 802, 24, 2, 66];
        radix_sort(&mut array);
        assert_eq!(array, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_merge_sort(){
        let mut array = vec![170, 45, 75, 90, 802, 24, 2, 66];
        merge_sort(&mut array);
        assert_eq!(array, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

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
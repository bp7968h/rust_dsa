pub fn linear_search<T: Ord + Copy>(arr: &[T], value: T) -> Option<usize>{
    for (index, item) in arr.iter().enumerate(){
        if *item == value{
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests{
    use super::*;

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
}
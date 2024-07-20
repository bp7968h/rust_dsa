use std::cmp::Ordering;

pub fn binary_search<T: Ord>(arr: &[T], find: T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut start_idx = 0;
    let mut end_idx = arr.len().wrapping_sub(1);

    while start_idx <= end_idx {
        let mid = start_idx + (end_idx - start_idx) / 2;
        match arr[mid].cmp(&find) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => start_idx = mid + 1,
            Ordering::Greater => {
                if mid == 0 { break; } // Prevents overflow
                end_idx = mid - 1;
            },
        }
    }

    None
}


#[cfg(test)]
mod tests{
    use super::*;

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
}
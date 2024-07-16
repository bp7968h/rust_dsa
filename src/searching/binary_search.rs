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

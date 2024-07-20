pub fn quick_sort<T: Ord + std::marker::Copy>(arr: &mut [T], start: isize, end: isize) {
    if start < end {
        let middle_point = parition(arr, start, end);
        //sort the left
        quick_sort(arr, start, middle_point);
        //sort the right
        quick_sort(arr, middle_point + 1, end);
    }
}

fn parition<T: Ord + std::marker::Copy>(arr: &mut [T], start: isize, end: isize) -> isize {
    let pivot_idx = start + (end - start) / 2;
    let pivot_elm = arr[pivot_idx as usize];

    let mut i = start - 1;
    let mut j = end + 1;
    // two pointer loop to position at the correct index and swap the element divided by pivot
    loop {
        loop {
            // Move right to find element greater than the pivot
            i += 1;
            if arr[i as usize] >= pivot_elm {
                break;
            }
        }

        loop {
            // Move left to find element greater than the pivot
            j -= 1;
            if arr[j as usize] <= pivot_elm {
                break;
            }
        }

        if i >= j {
            return j;
        }

        arr.swap(i as usize, j as usize);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut array = vec![4, 2, 5, 1, 3];
        let len = array.len();
        quick_sort(&mut array, 0, (len - 1) as isize);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
    }
}
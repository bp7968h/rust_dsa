pub fn merge_sort<T: Ord + Clone + Copy>(arr: &mut [T]){

    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    
    let mut ret = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);
    arr.copy_from_slice(&ret);
}

fn merge<T: Ord + Copy>(a: &[T], b: &[T], ret: &mut [T]){
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;
    
    
    while left < a.len() && right < b.len(){
        if a[left] <= b[right] {
            ret[index] = a[left];
            index += 1;
            left += 1;
        }else {
            ret[index] = b[right];
            index += 1;
            right += 1;
        }
    }
    
    if left < a.len(){
        ret[index..].copy_from_slice(&a[left..]);
    }
    if right < b.len(){
        ret[index..].copy_from_slice(&b[right..]);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_merge_sort(){
        let mut array = vec![170, 45, 75, 90, 802, 24, 2, 66];
        merge_sort(&mut array);
        assert_eq!(array, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }
}
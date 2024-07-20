pub fn counting_sort(arr: &mut [usize]){
    //find the max value
    let max_value: usize = match arr.iter().max() {
        Some(max) => *max,
        None => return,
    };
    //initialize a counting array
    let mut count_array = vec![0; max_value + 1];
    
    //fill the counting array
    for i in arr.iter(){
        count_array[*i] += 1;
    }

    //reconstruct the arry with sorted value
    let mut index = 0;
    for (number, &count) in count_array.iter().enumerate(){
        for _ in 0..count{
            arr[index] = number;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut array = vec![4, 2, 5, 2, 1, 1, 0, 3];
        counting_sort(&mut array);
        assert_eq!(array, vec![0, 1, 1, 2, 2, 3, 4, 5]);
    }
}
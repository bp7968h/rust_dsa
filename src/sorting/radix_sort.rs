pub fn radix_sort(arr: &mut [usize]) {
    // Find the maximum number to determine the number of digits
    let max_number = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    // Perform counting sort for every digit, starting with the least significant digit
    let mut digit_place = 1;
    while max_number / digit_place > 0 {
        counting_sort_by_digit(arr, digit_place);
        digit_place *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [usize], digit_place: usize) {
    let mut count = vec![0; 10]; // digits (0-9)
    let mut output = vec![0; arr.len()]; // Output array

    // Count occurrences of each digit
    for &num in arr.iter() {
        let digit = (num / digit_place) % 10;
        count[digit] += 1;
    }

    // Compute prefix sums to get actual positions
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build the output array using the counts
    for &num in arr.iter().rev() {
        let digit = (num / digit_place) % 10;
        count[digit] -= 1;
        output[count[digit]] = num;
    }

    // Copy the sorted numbers back into the original array
    for i in 0..arr.len() {
        arr[i] = output[i];
    }
}
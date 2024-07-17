use rust_dsa::sorting::{bubble_sort, selection_sort, insertion_sort, quick_sort, counting_sort, radix_sort, merge_sort};
use rust_dsa::searching::{linear_search, binary_search};
use rust_dsa::data_structure::{LinkedList};

fn main() {
    let mut bubble_array = vec![4, 2, 5, 1, 3];
    bubble_sort(&mut bubble_array);
    println!("Bubble sorted: {:?}", bubble_array);

    let mut selection_array = vec![4, 2, 5, 1, 3];
    selection_sort(&mut selection_array);
    println!("Selection sorted: {:?}", selection_array);

    let mut insertion_array = vec![4, 2, 5, 1, 3];
    insertion_sort(&mut insertion_array);
    println!("Insertion sorted: {:?}", insertion_array);

    let mut quick_array = vec![4, 2, 5, 1, 3];
    let len = quick_array.len();
    quick_sort(&mut quick_array, 0, (len -1) as isize);
    println!("Quick sorted: {:?}", quick_array);

    let mut counting_array: [usize; 16] = [5, 3, 8, 6, 2, 7, 4, 3, 8, 7, 2, 5, 9, 0, 1, 4];
    counting_sort(&mut counting_array);
    println!("Counting sorted: {:?}", counting_array);

    let mut radix_array = vec![ 33, 45, 105, 40, 25, 17, 24, 5];
    radix_sort(&mut radix_array);
    println!("Radix sorted: {:?}", radix_array);

    let mut merge_array = vec![4, 2, 5, 1, 3];
    merge_sort(&mut merge_array);
    println!("Merge sorted: {:?}", merge_array);

    let linear_search_array = vec![4, 2, 5, 1, 3];
    let find = 1;
    if let Some(index) = linear_search(&linear_search_array, find){
        println!("Found {} at index {}", find, index);
    }else{
        println!("Value {} not found in {:?}", find, linear_search_array);
    };

    let mut binary_search_array = vec![4, 2, 5, 1, 3];
    merge_sort(&mut binary_search_array);
    let find = 1;
    if let Some(index) = binary_search(&binary_search_array, find){
        println!("Found {} at index {}", find, index);
    }else {
        println!("Value {} not found in {:?}", find, linear_search_array);
    };

    let mut list = LinkedList::new(1);
    list.add(2);
    list.add(3);
    println!("LinkedList Structure: {}", list);
}

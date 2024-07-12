use rust_dsa::sorting::{bubble_sort};

fn main() {
    let mut array1 = vec![4, 2, 5, 1, 3];
    bubble_sort(&mut array1);
    println!("Bubble sorted: {:?}", array1);

}

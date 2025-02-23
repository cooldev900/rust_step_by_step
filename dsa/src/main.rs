mod search_algorithms;

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let target = 3;
    let result = search_algorithms::linear_search_algorithm::linear_search_algorithm(&arr, &target);
    match result {
        Some(index) => println!("Element found at index: {}", index),
        None => println!("Element not found"),
    }
}

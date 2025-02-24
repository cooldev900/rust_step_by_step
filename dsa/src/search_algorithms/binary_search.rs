pub fn iterative_binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    if arr.len() == 0 { return None; }
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if &arr[mid] == target {
            return Some(mid);
        } else if &arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_binary_search_found() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(iterative_binary_search(&arr, &5), Some(4));
    }

    #[test]
    fn test_iterative_binary_search_not_found() {
        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(iterative_binary_search(&arr, &11), None);
    }

    #[test]
    fn test_iterative_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(iterative_binary_search(&arr, &1), None);
    }

    #[test]
    fn test_iterative_binary_search_single_element_found() {
        let arr = [1];
        assert_eq!(iterative_binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_iterative_binary_search_single_element_not_found() {
        let arr = [1];
        assert_eq!(iterative_binary_search(&arr, &2), None);
    }

    #[test]
    fn test_iterative_binary_search_first_element() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(iterative_binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_iterative_binary_search_last_element() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(iterative_binary_search(&arr, &5), Some(4));
    }
}

pub fn linear_search_algorithm<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_algorithm_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search_algorithm(&arr, &3), Some(2));
    }

    #[test]
    fn test_linear_search_algorithm_not_found() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search_algorithm(&arr, &6), None);
    }

    #[test]
    fn test_linear_search_algorithm_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(linear_search_algorithm(&arr, &1), None);
    }

    #[test]
    fn test_linear_search_algorithm_first_element() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search_algorithm(&arr, &1), Some(0));
    }

    #[test]
    fn test_linear_search_algorithm_last_element() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(linear_search_algorithm(&arr, &5), Some(4));
    }

    #[test]
    fn test_linear_search_algorithm_multiple_occurrences() {
        let arr = [1, 2, 3, 2, 5];
        assert_eq!(linear_search_algorithm(&arr, &2), Some(1));
    }
}
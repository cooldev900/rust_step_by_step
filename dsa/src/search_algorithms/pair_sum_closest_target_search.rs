use std::ops::Add;

fn pair_sum_exactly_to_target<T>(first_arr: &[T], second_arr: &[T], target: &T) -> Option<[T; 2]>
where
    T: Copy + Ord + PartialEq + Add<Output = T>,
{
    // Return early if either array is empty.
    if first_arr.is_empty() || second_arr.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = second_arr.len() - 1;

    // Use '&&' so that both indices are within bounds.
    while left < first_arr.len() && right > 0 {
        let sum = first_arr[left] + second_arr[right];
        if &sum == target {
            return Some([first_arr[left], second_arr[right]]);
        } else if &sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    // Check the possibility when right is zero.
    if left < first_arr.len() {
        let sum = first_arr[left] + second_arr[0];
        if &sum == target {
            return Some([first_arr[left], second_arr[0]]);
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum_exactly_to_target() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [6, 7, 8, 9, 10];
        let target = 10;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), Some([1, 9]));

        let target = 15;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), Some([5, 10]));

        let target = 20;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), None);
    }

    #[test]
    fn test_pair_sum_with_empty_arrays() {
        let arr1: [i32; 0] = [];
        let arr2 = [1, 2, 3];
        let target = 4;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), None);

        let arr1 = [1, 2, 3];
        let arr2: [i32; 0] = [];
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), None);

        let arr1: [i32; 0] = [];
        let arr2: [i32; 0] = [];
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), None);
    }

    #[test]
    fn test_pair_sum_with_negative_numbers() {
        let arr1 = [-5, -3, -1, 1, 3, 5];
        let arr2 = [-10, -5, 0, 5, 10];
        let target = 0;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), Some([-5, 5]));

        let target = -15;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), Some([-5, -10]));

        let target = 15;
        assert_eq!(pair_sum_exactly_to_target(&arr1, &arr2, &target), Some([5, 10]));
    }
}

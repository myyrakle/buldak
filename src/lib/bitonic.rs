//! bitonic sort algorithm.
//!
//! This sort works only if the length of the array is 2^N.
//!
//! **O(Nlog₂N)**

/// Sort in ascending order using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21];
/// bitonic::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21];
/// bitonic::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21];
/// bitonic::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F) -> Result<(), String>
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _bitonic_sort_impl(array, compare)
}

fn _bitonic_sort_impl<T, F>(array: &mut [T], compare: F) -> Result<(), String>
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let len = array.len() as isize;

    if len != (len & -len) {
        Err("This sort works only if the length of the array is 2^N.".to_string())
    } else {
        _bitonic_sort_recursive(array, 0, array.len(), true, compare);
        Ok(())
    }
}

fn _bitonic_sort_recursive<T, F>(array: &mut [T], low: usize, count: usize, asc: bool, compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if count > 1 {
        let middle = count / 2;

        _bitonic_sort_recursive(array, low, middle, true, compare.clone());
        _bitonic_sort_recursive(array, low + middle, middle, false, compare.clone());

        _bitonic_merge(array, low, count, asc, compare);
    }
}

mod utils;

fn _bitonic_merge<T, F>(array: &mut [T], low: usize, count: usize, asc: bool, compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if count > 1 {
        let middle = count / 2;

        for i in low..(low + middle) {
            _compare_swap(array, i, i + middle, asc, compare.clone());
        }

        _bitonic_merge(array, low, middle, asc, compare.clone());
        _bitonic_merge(array, low + middle, middle, asc, compare.clone());
    }
}

fn _compare_swap<T, F>(array: &mut [T], i: usize, j: usize, asc: bool, compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if asc == (compare(&array[i], &array[j]) == std::cmp::Ordering::Greater) {
        utils::swap(array, i, j);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort_ascending() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = vec![TestCase {
            input: vec![1, 4, 2, 3, 5, 111, 234, 21, 13],
            expected: vec![1, 2, 3, 4, 5, 13, 21, 111, 234],
        }];

        for case in test_cases {
            let mut actual = case.input.clone();
            super::sort(&mut actual).unwrap();
            assert_eq!(actual, case.expected);
        }
    }

    #[test]
    fn sort_descending() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = vec![TestCase {
            input: vec![1, 4, 2, 3, 5, 111, 234, 21, 13],
            expected: vec![234, 111, 21, 13, 5, 4, 3, 2, 1],
        }];

        for case in test_cases {
            let mut actual = case.input.clone();
            super::sort_reverse(&mut actual).unwrap();
            assert_eq!(actual, case.expected);
        }
    }
}

//! heap sort algorithm.
//!
//! unstable sort  
//! **O(Nlog₂N)**

mod utils;

/// Sort in ascending order using a heap sort algorithm.
///
/// ```rust
/// use buldak::heap;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// heap::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a heap sort algorithm.
///
/// ```rust
/// use buldak::heap;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// heap::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a heap sort algorithm.
///
/// ```rust
/// use buldak::heap;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// heap::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _heap_sort_impl(array, compare)
}

fn _heap_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _make_heap(array, array.len(), compare.clone());
    for i in (0..array.len()).rev() {
        utils::swap(array, 0, i);
        _make_heap(array, i, compare.clone());
    }
}

fn _make_heap<T, F>(array: &mut [T], len: usize, compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in 1..len {
        let mut child = i;

        while child > 0 {
            let root = (child - 1) / 2;
            if compare(&array[root], &array[child]) == std::cmp::Ordering::Less {
                utils::swap(array, root, child);
            }
            child = root;
        }
    }
}

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
            super::sort(&mut actual);
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
            super::sort_reverse(&mut actual);
            assert_eq!(actual, case.expected);
        }
    }
}

//! stalin sort algorithm.
//!
//! Purge all unsorted elements and make them sorted.
//!
//! O(N)

/// Sort in ascending order using a stalin sort algorithm.
///
/// ```rust
/// use buldak::stalin;
///
/// let mut nums = vec![1, 4, 2, 3, 5, 11, 23, 21, 13, 0];
/// stalin::sort(&mut nums);
/// assert_eq!(nums, vec![1, 4, 5, 11, 23]);
/// ```
pub fn sort<T>(array: &mut Vec<T>)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a stalin sort algorithm.
///
/// ```rust
/// use buldak::stalin;
///
/// let mut nums = vec![1, 4, 2, 3, 5, 11, 23, 21, 13, 0];
/// stalin::sort_reverse(&mut nums);
/// assert_eq!(nums, vec![1, 0]);
/// ```
pub fn sort_reverse<T>(array: &mut Vec<T>)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a stalin sort algorithm.
///
/// ```rust
/// use buldak::stalin;
///
/// let mut nums = vec![1, 4, 2, 3, 5, 11, 23, 21, 13, 0];
/// stalin::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, vec![1, 4, 5, 11, 23]);
/// ```
pub fn sort_by<T, F>(array: &mut Vec<T>, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _stalin_sort_impl(array, compare)
}

fn _stalin_sort_impl<T, F>(array: &mut Vec<T>, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut prev = &array[0];
    *array = array
        .iter()
        .filter(|&e| {
            let cmp = compare(prev, e);
            if cmp != std::cmp::Ordering::Greater {
                prev = e;
                true
            } else {
                false
            }
        })
        .cloned()
        .collect();
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
            expected: vec![1, 4, 5, 111, 234],
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
            expected: vec![1],
        }];

        for case in test_cases {
            let mut actual = case.input.clone();
            super::sort_reverse(&mut actual);
            assert_eq!(actual, case.expected);
        }
    }
}

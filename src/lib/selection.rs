//! selection sort algorithm.
//!
//! unstable sort  
//! **O(N²)**

mod utils;

/// Sort in ascending order using a selection sort algorithm.
///
/// ```rust
/// use buldak::selection;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// selection::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a selection sort algorithm.
///
/// ```rust
/// use buldak::selection;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// selection::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a selection sort algorithm.
///
/// ```rust
/// use buldak::selection;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// selection::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _selection_sort_impl(array, compare)
}

fn _selection_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut last = array.len() - 1;

    while 0 != last {
        let mut max_i = 0;
        let mut i = 1;

        while i <= last {
            match compare(&array[i], &array[max_i]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => max_i = i,
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }

        utils::swap(array, last, max_i);

        last -= 1;
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

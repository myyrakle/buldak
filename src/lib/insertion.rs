//! insertion sort algorithm.
//!
//! stable sort  
//! **O(N²)**

/// Sort in ascending order using a insertion sort algorithm.
///
/// ```rust
/// use buldak::insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// insertion::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a insertion sort algorithm.
///
/// ```rust
/// use buldak::insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// insertion::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a insertion sort algorithm.
///
/// ```rust
/// use buldak::insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// insertion::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _insertion_sort_impl(array, compare)
}

fn _insertion_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut start = 1;
    let end = array.len();

    while start != end {
        let target = array[start].clone();

        let mut back = start as isize - 1;

        while back >= 0 {
            if compare(&target, &array[back as usize]) == std::cmp::Ordering::Less {
                array[(back + 1) as usize] = array[back as usize].clone();
            } else {
                break;
            }

            back -= 1;
        }

        array[(back + 1) as usize] = target;

        start += 1;
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

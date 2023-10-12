//! merge sort algorithm.
//!
//! stable sort  
//! **O(Nlog₂N)**

/// Sort in ascending order using a merge sort algorithm.
///
/// ```rust
/// use buldak::merge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// merge::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a merge sort algorithm.
///
/// ```rust
/// use buldak::merge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// merge::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a merge sort algorithm.
///
/// ```rust
/// use buldak::merge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// merge::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _merge_sort_impl(array, compare)
}

// implementation
fn _merge_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if array.len() == 0 {
        return;
    }

    let mut sorted: std::vec::Vec<T> = array.iter().map(|e| e.clone()).collect();
    _merge_sort_recursive(array, &mut sorted, 0, array.len() - 1, compare)
}

fn _merge_sort_recursive<T, F>(
    array: &mut [T],
    sorted: &mut [T],
    left: usize,
    right: usize,
    compare: F,
) where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if left < right {
        let middle = (left + right) / 2;
        _merge_sort_recursive(array, sorted, left, middle, compare.clone());
        _merge_sort_recursive(array, sorted, middle + 1, right, compare.clone());
        _merge(array, sorted, left, middle, right, compare);
        println!("{:?}", sorted);
    }
}

fn _merge<T, F>(
    array: &mut [T],
    sorted: &mut [T],
    left: usize,
    middle: usize,
    right: usize,
    compare: F,
) where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let mut l = left;
    let mut r = middle + 1;
    let mut sorted_index = left;

    while l <= middle && r <= right {
        match compare(&array[l], &array[r]) {
            std::cmp::Ordering::Greater => {
                sorted[sorted_index] = array[r].clone();
                sorted_index += 1;
                r += 1;
            }
            _ => {
                sorted[sorted_index] = array[l].clone();
                sorted_index += 1;
                l += 1;
            }
        }
    }

    if l > middle {
        for e in r..=right {
            sorted[sorted_index] = array[e].clone();
            sorted_index += 1;
        }
    } else {
        for e in l..=middle {
            sorted[sorted_index] = array[e].clone();
            sorted_index += 1;
        }
    }

    for e in left..=right {
        array[e] = sorted[e].clone();
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

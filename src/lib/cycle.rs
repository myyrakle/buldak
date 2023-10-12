//! cycle sort algorithm.
//!
//! unstable sort  
//! **O(N²)**

/// Sort in ascending order using a cycle sort algorithm.
///
/// ```rust
/// use buldak::cycle;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// cycle::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a cycle sort algorithm.
///
/// ```rust
/// use buldak::cycle;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// cycle::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a cycle sort algorithm.
///
/// ```rust
/// use buldak::cycle;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// cycle::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _cycle_sort_impl(array, compare)
}

fn _cycle_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let n = array.len();

    // traverse array elements and put it to on
    // the right place
    for cycle_start in 0..=(n - 2) {
        // initialize item as starting point
        let mut item = array[cycle_start].clone();

        // Find position where we put the item. We basically
        // count all smaller elements on right side of item.
        let mut pos = cycle_start;
        for i in (cycle_start + 1)..n {
            if compare(&array[i], &item) == std::cmp::Ordering::Less {
                pos += 1;
            }
        }

        // If item is already in correct position
        if pos == cycle_start {
            continue;
        }

        // ignore all duplicate  elements
        while compare(&item, &array[pos]) == std::cmp::Ordering::Equal {
            pos += 1;
        }

        // swap
        let temp = item.clone();
        item = array[pos].clone();
        array[pos] = temp;

        // Rotate rest of the cycle
        while pos != cycle_start {
            pos = cycle_start;

            // Find position where we put the element
            for i in (cycle_start + 1)..n {
                if compare(&array[i], &item) == std::cmp::Ordering::Less {
                    pos += 1;
                }
            }

            // ignore all duplicate  elements
            while compare(&item, &array[pos]) == std::cmp::Ordering::Equal {
                pos += 1;
            }

            // swap
            let temp = item.clone();
            item = array[pos].clone();
            array[pos] = temp;
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

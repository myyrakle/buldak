//! binary insertion sort algorithm.
//!
//! stable sort  
//! **O(N²)**

/// Sort in ascending order using a binary insertion sort algorithm.
///
/// ```rust
/// use buldak::binary_insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// binary_insertion::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a binary insertion sort algorithm.
///
/// ```rust
/// use buldak::binary_insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// binary_insertion::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a binary insertion sort algorithm.
///
/// ```rust
/// use buldak::binary_insertion;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// binary_insertion::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _binary_insertion_sort_impl(array, compare)
}

fn _binary_search<T, F>(array: &[T], item: &T, low: isize, high: isize, compare: F) -> isize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if high <= low {
        return if compare(item, &array[low as usize]) == std::cmp::Ordering::Greater {
            low + 1
        } else {
            low
        };
    }

    let middle = (low + high) / 2;

    match compare(item, &array[middle as usize]) {
        std::cmp::Ordering::Equal => middle + 1,
        std::cmp::Ordering::Greater => {
            _binary_search(array, item, middle + 1, high, compare.clone())
        }
        std::cmp::Ordering::Less => _binary_search(array, item, low, middle - 1, compare.clone()),
    }
}

fn _binary_insertion_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in 1..array.len() {
        let mut j = (i - 1) as isize;

        let target = array[i].clone();

        let index = _binary_search(array, &target, 0, j, compare.clone());

        while j >= index {
            array[(j + 1) as usize] = array[j as usize].clone();
            j -= 1;
        }
        array[(j + 1) as usize] = target;
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

//! stooge sort algorithm.
//!
//! **O(N<sup>2.7095...</sup>)**

mod utils;

/// Sort in ascending order using a stooge sort algorithm.
///
/// ```rust
/// use buldak::stooge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// stooge::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a stooge sort algorithm.
///
/// ```rust
/// use buldak::stooge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// stooge::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a stooge sort algorithm.
///
/// ```rust
/// use buldak::stooge;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// stooge::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _stooge_sort_impl(array, compare);
}

fn _stooge_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _stooge_sort_recursive(array, 0, array.len() - 1, compare);
}

fn _stooge_sort_recursive<T, F>(array: &mut [T], i: usize, j: usize, compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    // If the leftmost element is larger than the rightmost element
    if compare(&array[i], &array[j]) == std::cmp::Ordering::Greater {
        utils::swap(array, i, j);
    }

    // If there are at least 3 elements in the array
    if j - i + 1 > 2 {
        let k = (j - i + 1) / 3;
        _stooge_sort_recursive(array, i, j - k, compare.clone()); // Sort the first 2/3 of the array
        _stooge_sort_recursive(array, i + k, j, compare.clone()); // Sort the last 2/3 of the array
        _stooge_sort_recursive(array, i, j - k, compare); // Sort the first 2/3 of the array again
    }
}

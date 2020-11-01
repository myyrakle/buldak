//! bitonic sort algorithm.
//!
//! **O(N²)**

/// Sort in ascending order using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bitonic::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bitonic::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a bitonic sort algorithm.
///
/// ```rust
/// use buldak::bitonic;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bitonic::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _bitonic_sort_impl(array, compare)
}

fn _bitonic_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _bitonic_sort_recursive(array, 0, array.len(), true, compare)
}

fn _bitonic_sort_recursive<T, F>(array: &mut [T], left: usize, right: usize, asc: bool, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if right > 1 {
        let middle = right / 2;

        _bitonic_sort_recursive(array, left, middle, true, compare.clone());
        _bitonic_sort_recursive(array, left + middle, middle, false, compare.clone());

        _bitonic_merge(array, left, right, asc, compare)
    }
}

mod utils;

fn _bitonic_merge<T, F>(array: &mut [T], left: usize, right: usize, asc: bool, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if right > 1 {
        let middle = right / 2;

        for i in left..(left + middle) {
            if (compare(&array[i], &array[i + middle]) == std::cmp::Ordering::Greater) == asc {
                utils::swap(array, i, i + middle);
            }
        }

        _bitonic_merge(array, left, middle, asc, compare.clone());
        _bitonic_merge(array, left + middle, middle, asc, compare.clone());
    }
}

//! pancake sort algorithm.
//!  
//! unstable sort  
//! **O(N²)**

mod utils;

/// Sort in ascending order using a oddeven sort algorithm.
///
/// ```rust
/// use buldak::pancake;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pancake::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```

pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a oddeven sort algorithm.
///
/// ```rust
/// use buldak::pancake;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pancake::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
///
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a oddeven sort algorithm.
///
/// ```rust
/// use buldak::pancake;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pancake::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
///
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _pancake_sort_impl(array, compare)
}

// Returns index of the
// maximum element in
// arr[0..n-1]
fn _find_max<T, F>(array: &[T], n: usize, compare: F) -> usize
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut max_index = 0;
    for i in 0..n {
        if compare(&array[i], &array[max_index]) == std::cmp::Ordering::Greater {
            max_index = i;
        }
    }
    max_index
}

// Reverses arr[0..i]
fn _flip<T>(array: &mut [T], i: usize) {
    let mut i = i;

    let mut start = 0;
    while start < i {
        utils::swap(array, start, i);
        start += 1;
        i -= 1;
    }
}

fn _pancake_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    // Start from the complete
    // array and one by one
    // reduce current size by one
    for i in (2..array.len()).rev() {
        // Find index of the
        // maximum element in
        // arr[0..current_size-1]
        let max_index = _find_max(array, i, compare.clone());

        // Move the maximum
        // element to end of
        // current array if
        // it's not already
        // at the end
        if max_index != i {
            // To move at the end,
            // first move maximum
            // number to beginning
            _flip(array, max_index);

            // Now move the maximum
            // number to end by
            // reversing current array
            _flip(array, i);
        }
    }
}

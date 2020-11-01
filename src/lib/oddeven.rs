//! oddeven sort algorithm.
//!
//! Algorithm optimized for parallel processors.  
//!  
//! **O(N²)**

mod utils;

/// Sort in ascending order using a oddeven sort algorithm.
///
/// ```rust
/// use buldak::oddeven;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// oddeven::sort(&mut nums);
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
/// use buldak::oddeven;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// oddeven::sort_reverse(&mut nums);
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
/// use buldak::oddeven;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// oddeven::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
///
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _oddeven_sort_impl(array, compare)
}

fn _oddeven_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut sorted = false;

    while !sorted {
        sorted = true;

        let mut i = 1;
        while i < array.len() - 1 {
            if compare(&array[i], &array[i + 1]) == std::cmp::Ordering::Greater {
                utils::swap(array, i, i + 1);
                sorted = false;
            }
            i += 2;
        }

        let mut i = 0;
        while i < array.len() - 1 {
            if compare(&array[i], &array[i + 1]) == std::cmp::Ordering::Greater {
                utils::swap(array, i, i + 1);
                sorted = false;
            }
            i += 2;
        }
    }
}

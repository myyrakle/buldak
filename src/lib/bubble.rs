//! bubble sort algorithm.
//!
//! stable sort
//! **O(N²)**

mod utils;

/// Sort in ascending order using a bubble sort algorithm.
///
/// ```rust
/// use buldak::bubble;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bubble::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```

pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a bubble sort algorithm.
///
/// ```rust
/// use buldak::bubble;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bubble::sort_reverse(&mut nums);
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
/// and sorts it using a bubble sort algorithm.
///
/// ```rust
/// use buldak::bubble;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// bubble::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
///
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    _bubble_sort_impl(array, compare)
}

fn _bubble_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut last = array.len();

    while 0 != last {
        let mut i = 0;

        while (i + 1) < last {
            match compare(&array[i], &array[i + 1]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => utils::swap(array, i, i + 1),
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }
        last -= 1;
    }
}

//! tim sort algorithm.
//!
//! **O(Nlog₂N)**

// not impl

mod utils;

/// Sort in ascending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    make_heap(array, array.len(), compare.clone());
    for i in (0..array.len()).rev() {
        utils::swap(array, 0, i);
        make_heap(array, i, compare.clone());
    }
}

fn make_heap<T, F>(array: &mut [T], len: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in 1..len {
        let mut child = i;

        while child > 0 {
            let root = (child - 1) / 2;
            if compare(&array[root], &array[child]) == std::cmp::Ordering::Less {
                utils::swap(array, root, child);
            }
            child = root;
        }
    }
}
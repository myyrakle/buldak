//! quick sort algorithm.
//!
//! unstable sort  
//! **average:O(Nlog₂N), worst:O(N²)**

//mod utils;

/// Sort in ascending order using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _quick_sort_impl(array, compare)
}

fn _library_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
}

fn _rebalance<T, F>(array: &mut [T], begin:usize, end:usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let r = end;
    let w = end/2;

    while r>=begin {
        //array[w+]
    }
}
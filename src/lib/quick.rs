//! quick sort algorithm.
//!
//! **average:O(Nlog₂N), worst:O(N²)**

mod utils;

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

fn _quick_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if array.len() == 0 {
        return;
    }
    _quick_sort_recursive(array, 0, array.len() - 1, compare)
}

// implementation

// recurive
fn _quick_sort_recursive<T, F>(array: &mut [T], left: usize, right: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if left >= right {
        return;
    }

    let pivot = _quick_partition(array, left, right, compare.clone());

    _quick_sort_recursive(array, left, pivot, compare.clone());
    _quick_sort_recursive(array, pivot + 1, right, compare);
}

fn _quick_partition<T, F>(array: &mut [T], left: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let pivot = array[left].clone();
    let mut l = left;
    let mut r = right;

    while l < r {
        while compare(&pivot, &array[r]) == std::cmp::Ordering::Less {
            r -= 1;
        }

        while l < r && compare(&pivot, &array[l]) != std::cmp::Ordering::Less {
            l += 1;
        }

        utils::swap(array, l, r);
    }

    array[left] = array[l].clone();
    array[l] = pivot;

    return l;
}

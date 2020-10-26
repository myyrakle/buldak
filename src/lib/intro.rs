//! intro sort algorithm.
//!
//! **O(Nlog₂N)**

// not impl

mod utils;

/// Sort in ascending order using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _intro_sort_impl(array, compare)
}

fn _intro_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let max_depth = (array.len() as f64).log2().floor() as isize * 2;
    _intro_sort_recursive(array, 0, array.len()-1, max_depth, compare)
}

fn _intro_sort_recursive<T, F>(array: &mut [T], begin:usize, end:usize, max_depth: isize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    println!("{}, {}", begin, end);

    if begin>=end {
        return ();
    }
    let n = end-begin;

    if n <= 1 {
        return ();
    } else if max_depth == 0 {
        _heap_sort(array, begin, end, compare)
    } else {
        let pivot = _quick_partition(array, begin, end, compare.clone());
        println!("{}", pivot);
        _intro_sort_recursive(array, 0, pivot, max_depth-1, compare.clone());
        _intro_sort_recursive(array, pivot+1, n, max_depth-1, compare);
    }
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

fn _heap_sort<T, F>(array: &mut [T], begin:usize, end:usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _make_heap(array, begin, end, compare.clone());
    for i in (begin..end).rev() {
        utils::swap(array, begin, i);
        _make_heap(array, begin, i, compare.clone());
    }
}

fn _make_heap<T, F>(array: &mut [T], begin:usize, end:usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in (begin+1)..(end) {
        let mut child = i;

        while child > begin {
            let root = (child - 1) / 2;
            if compare(&array[root], &array[child]) == std::cmp::Ordering::Less {
                utils::swap(array, root, child);
            }
            child = root;
        }
    }
}

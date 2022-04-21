//! Pattern-defeating quick sort algorithm.
//!
//! unstable sort  
//! **best:O(N), average:O(Nlog₂N), worst:O(Nlog₂N)**
//!

// 참조 https://github.com/orlp/pdqsort/blob/master/pdqsort.h
// https://docs.rs/pdqsort/latest/src/pdqsort/lib.rs.html#1

mod utils;

/// Sort in ascending order using a quick sort algorithm.
///
/// ```rust
/// use buldak::pd_quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pd_quick::sort(&mut nums);
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
/// use buldak::pd_quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pd_quick::sort_reverse(&mut nums);
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
/// use buldak::pd_quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// pd_quick::sort_by(&mut nums, |l, r| l.cmp(r));
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

    let limit = std::mem::size_of::<usize>() * 8 - array.len().leading_zeros() as usize;

    _quick_sort_recursive(array, 0, array.len() - 1, compare)
}

// implementation

// recurive
fn _quick_sort_recursive<T, F>(array: &mut [T], left: usize, right: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    // Slices of up to this length get sorted using insertion sort.
    const MAX_INSERTION: usize = 20;

    // True if the last partitioning was reasonably balanced.
    let mut was_balanced = true;
    // True if the last partitioning didn't shuffle elements (the slice was already partitioned).
    let mut was_partitioned = true;

    loop {
        let len = v.len();

        // Very short slices get sorted using insertion sort.
        if len <= MAX_INSERTION {
            insertion_sort(v, is_less);
            return;
        }

        // If too many bad pivot choices were made, simply fall back to heapsort in order to
        // guarantee `O(n log n)` worst-case.
        if limit == 0 {
            heapsort(v, is_less);
            return;
        }

        // If the last partitioning was imbalanced, try breaking patterns in the slice by shuffling
        // some elements around. Hopefully we'll choose a better pivot this time.
        if !was_balanced {
            break_patterns(v);
            limit -= 1;
        }

        // Choose a pivot and try guessing whether the slice is already sorted.
        let (pivot, likely_sorted) = choose_pivot(v, is_less);

        // If the last partitioning was decently balanced and didn't shuffle elements, and if pivot
        // selection predicts the slice is likely already sorted...
        if was_balanced && was_partitioned && likely_sorted {
            // Try identifying several out-of-order elements and shifting them to correct
            // positions. If the slice ends up being completely sorted, we're done.
            if partial_insertion_sort(v, is_less) {
                return;
            }
        }

        // If the chosen pivot is equal to the predecessor, then it's the smallest element in the
        // slice. Partition the slice into elements equal to and elements greater than the pivot.
        // This case is usually hit when the slice contains many duplicate elements.
        if let Some(p) = pred {
            if !is_less(p, &v[pivot]) {
                let mid = partition_equal(v, pivot, is_less);

                // Continue sorting elements greater than the pivot.
                v = &mut { v }[mid..];
                continue;
            }
        }

        // Partition the slice.
        let (mid, was_p) = partition(v, pivot, is_less);
        was_balanced = cmp::min(mid, len - mid) >= len / 8;
        was_partitioned = was_p;

        // Split the slice into `left`, `pivot`, and `right`.
        let (left, right) = { v }.split_at_mut(mid);
        let (pivot, right) = right.split_at_mut(1);
        let pivot = &pivot[0];

        // Recurse into the shorter side only in order to minimize the total number of recursive
        // calls and consume less stack space. Then just continue with the longer side (this is
        // akin to tail recursion).
        if left.len() < right.len() {
            recurse(left, is_less, pred, limit);
            v = right;
            pred = Some(pivot);
        } else {
            recurse(right, is_less, Some(pivot), limit);
            v = left;
        }
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

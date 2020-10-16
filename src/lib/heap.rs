//! heap sort algorithm.
//!
//! **O(Nlog₂N)**

/// Sort in ascending order using a heap sort algorithm.
/// 
/// ```rust
/// use buldak::heap;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// heap::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    _heap_sort_impl(array, false)
}

/// Sort in descending order using a heap sort algorithm.
/// 
/// ```rust
/// use buldak::heap;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// heap::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    _heap_sort_impl(array, true)
}

fn _heap_sort_impl<T>(array: &mut [T], reverse: bool)
where
    T: std::cmp::Ord + std::clone::Clone,
{
    use std::collections::BinaryHeap;

    let heap: BinaryHeap<T> = array.iter().map(|e| e.clone()).collect();

    let sorted = heap.into_sorted_vec();

    let mut i = 0;
    let len = array.len();
    while i < array.len() {
        let index = if reverse { len - 1 - i } else { i };
        array[i] = sorted[index].clone();
        i += 1;
    }
}

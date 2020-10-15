//! quick sort algorithm

mod utils;

/**
Sort in ascending order using a quick sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
quick::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(arr, |l, r| l.cmp(r))
}

/** Sort in descending order using a quick sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
quick::sort_reverse(&mut nums);
assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
```
*/
pub fn sort_reverse<T>(arr: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(arr, |l, r| l.cmp(r).reverse())
}

/**
It takes a comparator function to determine the order,
and sorts it using a quick sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
quick::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if arr.len() == 0 {
        return;
    }
    _quick_impl(arr, 0, arr.len() - 1, compare)
}

// implementation

// recurive
fn _quick_impl<T, F>(arr: &mut [T], left: usize, right: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if left >= right {
        return;
    }

    let pivot = _quick_partition(arr, left, right, compare.clone());

    _quick_impl(arr, left, pivot, compare.clone());
    _quick_impl(arr, pivot + 1, right, compare);
}

fn _quick_partition<T, F>(arr: &mut [T], left: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let pivot = arr[left].clone();
    let mut l = left;
    let mut r = right;

    while l < r {
        while compare(&pivot, &arr[r]) == std::cmp::Ordering::Less {
            r -= 1;
        }

        while l < r && compare(&pivot, &arr[l]) != std::cmp::Ordering::Less {
            l += 1;
        }

        utils::swap(arr, l, r);
    }

    arr[left] = arr[l].clone();
    arr[l] = pivot;

    return l;
}

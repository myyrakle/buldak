//! merge sort algorithm

mod utils;

/**
Sort in ascending order using a merge sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
merge::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/** Sort in descending order using a merge sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
merge::sort_reverse(&mut nums);
assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
```
*/
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/**
It takes a comparator function to determine the order,
and sorts it using a merge sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
merge::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if array.len() == 0 {
        return;
    }
    _merge_impl(array, 0, array.len() - 1, compare)
}

// implementation
fn _merge_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
}

//! selection sort algorithm

mod utils;

/**
Sort in ascending order using a selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
selection::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(arr, |l, r| l.cmp(r))
}

/** Sort in descending order using a selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
selection::sort_reverse(&mut nums);
assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
```
*/
pub fn sort_reverse<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(arr, |l, r| l.cmp(r).reverse())
}

/**
It takes a comparator function to determine the order,
and sorts it using a selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
selection::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut last = arr.len() - 1;

    while 0 != last {
        let mut max_i = 0;
        let mut i = 1;

        while i <= last {
            match compare(&arr[i], &arr[max_i]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => max_i = i,
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }

        utils::swap(arr, last, max_i);

        last -= 1;
    }
}

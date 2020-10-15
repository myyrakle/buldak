//! insertion sort algorithm

mod utils;

/**
Sort in ascending order using a insertion sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
insertion::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(arr, |l, r| l.cmp(r))
}

/** Sort in descending order using a insertion sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
insertion::sort_reverse(&mut nums);
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
and sorts it using a insertion sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
insertion::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut start = 1;
    let end = arr.len();

    while start != end {
        let target = arr[start].clone();

        let mut back = start - 1;

        while back != 0 && compare(&target, &arr[back]) == std::cmp::Ordering::Less {
            arr[back + 1] = arr[back].clone();
            back -= 1;
        }

        if compare(&target, &arr[start]) == std::cmp::Ordering::Less {
            arr[back + 1] = arr[back].clone();
        } else {
            back += 1;
        }

        arr[back] = target;

        start += 1;
    }
}

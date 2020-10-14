//! smart bubble sort algorithm
//! This algorithm eliminates unnecessary repetitions in bubble sort.

mod utils;

/**
Sort in ascending order using a smart bubble sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
smart_bubble::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(arr, |l, r| l.cmp(r))
}

/** Sort in descending order using a smart bubble sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
smart_bubble::sort_reverse(&mut nums);
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
and sorts it using a smart bubble sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
smart_bubble::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut last = arr.len();

    while 0 != last {
        let mut swap_flag = false;
        let mut i = 0;

        while (i + 1) < last {
            match compare(&arr[i], &arr[i + 1]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => {
                    utils::swap(arr, i, i + 1);
                    swap_flag = true;
                }
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }
        last -= 1;

        if swap_flag == false {
            break;
        }
    }
}

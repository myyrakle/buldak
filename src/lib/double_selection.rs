//! double selection sort algorithm

mod utils;

/**
Sort in ascending order using a double selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
double_selection::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/** Sort in descending order using a double selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
double_selection::sort_reverse(&mut nums);
assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
```
*/
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/**
It takes a comparator function to determine the order,
and sorts it using a double selection sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
double_selection::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut first = 0;
    let mut last = array.len() - 1;

    while first <= last {
        let mut min_i = first;
        let mut max_i = first;
        let mut i = first;

        while i <= last {
            match compare(&array[i], &array[max_i]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => max_i = i,
                std::cmp::Ordering::Equal => (),
            }

            match compare(&array[i], &array[min_i]) {
                std::cmp::Ordering::Less => min_i = i,
                std::cmp::Ordering::Greater => (),
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }

        if first == max_i {
            max_i = min_i;
        }
        utils::swap(array, first, min_i);
        utils::swap(array, last, max_i);

        first += 1;
        last -= 1;
    }
}

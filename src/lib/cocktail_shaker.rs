//! cocktail shaker sort algorithm

mod utils;

/**
Sort in ascending order using a cocktail shaker sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
cocktail_shaker.rs::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/** Sort in descending order using a cocktail shaker sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
cocktail_shaker.rs::sort_reverse(&mut nums);
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
and sorts it using a cocktail shaker sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
cocktail_shaker.rs::sort_by(&mut nums, |l, r| l.cmp(r));
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut first = 0;
    let mut last = array.len();

    let mut shift = first;
    while first < last {
        let mut i = first + 1;

        while i < last {
            match compare(&array[i - 1], &array[i]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => {
                    utils::swap(array, i - 1, i);
                    shift = i;
                }
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }
        last = shift;

        let mut i = last - 1;
        while i > first {
            match compare(&array[i - 1], &array[i]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => {
                    utils::swap(array, i - 1, i);
                    shift = i;
                }
                std::cmp::Ordering::Equal => (),
            }
            i -= 1;
        }
        first = shift;
    }
}

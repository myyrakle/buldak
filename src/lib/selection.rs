mod utils;

pub fn selection_sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    selection_sort_by(arr, |l, r| l.cmp(r))
}

pub fn selection_sort_reverse<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    selection_sort_by(arr, |l, r| l.cmp(r).reverse())
}

/// bubble sort.
/// ```
/// bubble_sort_by(arr, |l, r| l.cmp(r));
/// ```
pub fn selection_sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut last = arr.len();

    while 0 != last {
        let mut i = 0;

        while (i + 1) < last {
            match compare(&arr[i], &arr[i + 1]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => utils::swap(arr, i, i + 1),
                std::cmp::Ordering::Equal => (),
            }
            i += 1;
        }
        last -= 1;
    }
}

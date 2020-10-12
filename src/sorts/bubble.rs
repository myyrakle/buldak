use super::utils;

pub fn bubble_sort<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    bubble_sort_by(arr, |l, r| l.cmp(r))
}

pub fn bubble_sort_reverse<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
    let len = arr.len();
    let mut i = 0;

    while (i + 1) < len {
        let mut j = i;

        while (j + 1) < len {
            if arr[j] < arr[j + 1] {
                utils::swap(arr, j, j + 1)
            }
            j += 1;
        }
        i += 1;
    }
}

pub fn bubble_sort_by<T, F>(arr: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let len = arr.len();
    let mut i = 0;

    while (i + 1) < len {
        let mut j = i;

        while (j + 1) < len {
            match compare(&arr[j], &arr[j + 1]) {
                std::cmp::Ordering::Less => (),
                std::cmp::Ordering::Greater => utils::swap(arr, j, j + 1),
                std::cmp::Ordering::Equal => (),
            }
            j += 1;
        }
        i += 1;
    }
}

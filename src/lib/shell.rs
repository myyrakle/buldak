//! shell sort algorithm.
//!
//! unstable sort
//! **O(N²)**

/// Sort in ascending order using a shell sort algorithm.
///
/// ```rust
/// use buldak::shell;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// shell::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a shell sort algorithm.
///
/// ```rust
/// use buldak::shell;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// shell::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a shell sort algorithm.
///
/// ```rust
/// use buldak::shell;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// shell::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _shell_sort_impl(array, compare);
}

fn _shell_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let mut gap = 1;

    while gap < array.len() {
        gap = gap * 3 + 1;
    }
    gap /= 3;

    while gap > 0 {
        for i in gap..array.len() {
            let mut k = (i - gap) as isize;
            let key = array[i].clone();
            while k >= 0 && compare(&key, &array[k as usize]) == std::cmp::Ordering::Less {
                array[k as usize + gap] = array[k as usize].clone();
                k -= gap as isize;
            }
            array[(k + gap as isize) as usize] = key;
        }

        gap /= 3;
    }

    // while gap > 0 {
    //     for i in gap..array.len() {
    //         let mut j = i - gap;

    //         let key = array[i].clone();

    //         while j>=0 && compare(&key, &array[j]) == std::cmp::Ordering::Less {
    //             array[j+gap] = array[j].clone();

    //             if j<gap {
    //                 break;
    //             }
    //             j-=gap;
    //         }
    //         array[j+gap] = key;
    //     }

    //     gap = gap/3+1;
    // }

    // let mut gap = array.len()/3 + 1;

    // while gap > 0 {
    //     for i in 0..gap {
    //         _shell_sort_insertion_impl(array, i, array.len()-1, gap, compare.clone());
    //     }

    //     gap = (gap/3) + 1;
    // }
}

// fn _shell_sort_insertion_impl<T, F>(array: &mut [T], first: usize, last: usize, gap: usize, compare: F)
// where
//     T: std::cmp::Ord + std::clone::Clone,
//     F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
// {
//     let mut i = first+gap;
//     while i<=last {
//         let key = array[i].clone();

//         let mut j = i-gap;
//         while j>=first && compare(&array[j], &key) == std::cmp::Ordering::Greater {
//             array[j + gap] = array[j].clone();

//             if j >= gap {
//                 j -= gap;
//             }
//         }

//         array[j+gap] = key.clone();

//         i += gap;
//     }
// }

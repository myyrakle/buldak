//! tim sort algorithm.
//!
//! **O(Nlog₂N)**

// not impl

mod utils;

/// Sort in ascending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    
}

fn _merge<T, F>(
    array: &mut [T],
    sorted: &mut [T],
    left: usize,
    middle: usize,
    right: usize,
    compare: F,
) where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let mut l = left;
    let mut r = middle + 1;
    let mut sorted_index = left;

    while l <= middle && r <= right {
        match compare(&array[l], &array[r]) {
            std::cmp::Ordering::Greater => {
                sorted[sorted_index] = array[r].clone();
                sorted_index += 1;
                r += 1;
            }
            _ => {
                sorted[sorted_index] = array[l].clone();
                sorted_index += 1;
                l += 1;
            }
        }
    }

    if l > middle {
        for e in r..=right {
            sorted[sorted_index] = array[e].clone();
            sorted_index += 1;
        }
    } else {
        for e in l..=middle {
            sorted[sorted_index] = array[e].clone();
            sorted_index += 1;
        }
    }

    for e in left..=right {
        array[e] = sorted[e].clone();
    }
}
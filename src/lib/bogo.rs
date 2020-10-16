//! bogo sort algorithm.
//!
//! **best:O(1), worst:O(∞)**

/// Sort in ascending order using a bogo sort algorithm.
/// 
/// ```rust
/// use buldak::bogo;
///
/// let mut nums = [5, 2, 3, 4, 1];
/// bogo::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a bogo sort algorithm.
/// 
/// ```rust
/// use buldak::bogo;
///
/// let mut nums = [5, 2, 3, 4, 1];
/// bogo::sort_reverse(&mut nums);
/// assert_eq!(nums, [5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a bogo sort algorithm.
/// 
/// ```rust
/// use buldak::bogo;
///
/// let mut nums = [5, 2, 3, 4, 1];
/// bogo::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    while sorted(array, &compare) == false {
        shuffle(array);
    }
}

fn sorted<T, F>(array: &[T], compare: &F) -> bool
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut current = &array[0];

    for e in array {
        match compare(current, e) {
            std::cmp::Ordering::Greater => return false,
            _ => (),
        }

        current = e;
    }

    return true;
}

use rand::seq::SliceRandom;

fn shuffle<T>(array: &mut [T]) {
    let mut rng = rand::thread_rng();
    array.shuffle(&mut rng);
}


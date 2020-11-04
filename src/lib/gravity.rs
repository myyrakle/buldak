//! gravity sort algorithm.
//!
//! Also called 'bead sort'  
//!
//! **O(S)**  
//! Dropping each and every bead’ as a separate operation since S is the sum of all the beads.  
//! very slow

/// Sort in ascending order using a sleep sort algorithm.
///
/// ```rust
/// use buldak::gravity;
///
/// let mut nums = [6, 4, 2, 3, 1, 5];
/// gravity::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn sort<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::cmp::Ord
        + std::convert::TryInto<usize>
        + std::convert::TryFrom<usize>
        + std::clone::Clone,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    _gravity_sort_impl(array, true)
}

/// Sort in descending order using a gravity algorithm.
///
/// ```rust
/// use buldak::gravity;
///
/// let mut nums = [6, 4, 2, 3, 1, 5];
/// gravity::sort_reverse(&mut nums);
/// assert_eq!(nums, [6, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::cmp::Ord
        + std::convert::TryInto<usize>
        + std::convert::TryFrom<usize>
        + std::clone::Clone,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    _gravity_sort_impl(array, false)
}

fn _gravity_sort_impl<T>(array: &mut [T], asc: bool) -> Result<(), String>
where
    T: std::cmp::Ord
        + std::convert::TryInto<usize>
        + std::convert::TryFrom<usize>
        + std::clone::Clone,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    // Find the maximum element
    let max: usize = array.iter().cloned().max().unwrap().try_into().unwrap();

    let mut beads = vec![false; max * array.len()];

    // mark the beads
    for i in 0..array.len() {
        let len: usize = array[i].to_owned().try_into().unwrap();
        for j in 0..len {
            beads[i * max + j] = true;
        }
    }

    for j in 0..max {
        // count how many beads are on each post
        let mut sum = 0;
        for i in 0..array.len() {
            if beads[i * max + j] {
                sum += 1;
            }
            beads[i * max + j] = false;
        }

        // Move beads down
        for i in (array.len() - sum)..array.len() {
            beads[i * max + j] = true;
        }
    }

    // Put sorted values in array using beads
    if asc {
        for i in 0..array.len() {
            let mut j = 0;
            while j < max && beads[i * max + j] {
                j += 1;
            }
            array[i] = T::try_from(j).unwrap();
        }
    } else {
        for i in 0..array.len() {
            let mut j = 0;
            while j < max && beads[i * max + j] {
                j += 1;
            }
            array[array.len() - 1 - i] = T::try_from(j).unwrap();
        }
    }

    return Ok(());
}

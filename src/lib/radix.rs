//! radix sort algorithm.
//!
//! **O(N)**

use std::convert::{TryFrom, TryInto};

/// Sort in ascending order using a counting sort algorithm.
///
/// The parameter 'max' is the absolute maximum value of the received array.
/// Any elements beyond this value will result in an error.
///
/// The parameter 'signed' chooses whether to support negative numbers.
/// When using only natural numbers, it is twice as efficient to set signed to false.
/// If signed is set to false and the array contains negative elements, an error occurs.
///
/// ```rsut
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// counting::sort(&mut nums, 300, true);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T, Max>(array: &mut [T])
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone + std::default::Default,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    //_radix_sort_impl(array, max, true, signed)
}

/// Sort in descending order using a bubble counting algorithm.
///
/// The parameter 'max' is the absolute maximum value of the received array.
/// Any elements beyond this value will result in an error.
///
/// The parameter 'signed' chooses whether to support negative numbers.
/// When using only natural numbers, it is twice as efficient to set signed to false.
/// If signed is set to false and the array contains negative elements, an error occurs.
///
/// ```rust
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// counting::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T, Max>(array: &mut [T])
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone + std::default::Default,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    //_radix_sort_impl(array, max, false, signed)
}

fn _radix_sort_impl<T, Max>(array: &mut [T], digits_max: usize, radix: usize)
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone + std::default::Default,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    let mut counts = vec![0; radix];
    let mut buffer: std::vec::Vec<T> = vec![std::default::Default::default(); array.len()];

    for n in 0..digits_max {
        for e in counts {
            e = 0;
        }

        let pval = radix.pow(n as u32);

        for e in array.iter() {
            let e: isize = e.to_owned().try_into().unwrap();
            let index = (e as usize / pval) % radix;
            counts[index] += 1;
        }

        {
            let mut i = 1;
            while i < radix {
                counts[i] = counts[i] + counts[i - 1];
                i += 1;
            }
        }

        for e in array.iter().rev() {
            let e: isize = e.to_owned().try_into().unwrap();
            let index = (e as usize / pval) % radix;
            buffer[counts[index] - 1] = e.to_owned().try_into().unwrap();
            counts[index] -= 1;
        }

        for i in 0..array.len() {
            array[i] = buffer[i].clone();
        }
    }
}

//! radix sort algorithm.
//!
//! **O(wN)**: w=length of key

use std::convert::{TryFrom, TryInto};

/// Sort in ascending order using a radix sort algorithm.
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
/// radix::sort(&mut nums, 300, true);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T], digits_max: usize, radix: usize)
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    _radix_sort_impl(array, digits_max, radix, true)
}

/// Sort in descending order using a radix algorithm.
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
/// radix::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T], digits_max: usize, radix: usize)
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    _radix_sort_impl(array, digits_max, radix, false)
}

fn _radix_sort_impl<T>(array: &mut [T], digits_max: usize, radix: usize, asc: bool)
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    use std::collections::LinkedList;
    let mut counter = vec![LinkedList::new(); radix];
    let mut neg_counter = vec![LinkedList::new(); radix];

    for y in 0..digits_max {
        for j in 0..array.len() {
            let mut e = array[j].to_owned().try_into().unwrap();
            let is_neg = e < 0;

            if is_neg {
                e = e.abs();
            }

            let modulo = radix.pow(y as u32 + 1) as isize;
            let divisor = modulo / radix as isize;
            let index = e % modulo / divisor;

            if is_neg {
                neg_counter[index as usize].push_back(array[j].to_owned());
            } else {
                counter[index as usize].push_back(array[j].to_owned());
            }
        }

        let mut pos = 0;

        if asc {
            for i in 0_isize..(neg_counter.len() as isize) {
                while let Some(value) = neg_counter[i as usize].pop_back() {
                    array[pos] = value;
                    pos += 1;
                }
            }
            for i in 0_isize..(counter.len() as isize) {
                while let Some(value) = counter[i as usize].pop_front() {
                    array[pos] = value;
                    pos += 1;
                }
            }
        } else {
            for i in 0_isize..(counter.len() as isize) {
                while let Some(value) = counter[i as usize].pop_back() {
                    array[pos] = value;
                    pos += 1;
                }
            }
            for i in 0_isize..(neg_counter.len() as isize) {
                while let Some(value) = neg_counter[i as usize].pop_front() {
                    array[pos] = value;
                    pos += 1;
                }
            }
        }
    }
}

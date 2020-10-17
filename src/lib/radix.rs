//! radix sort algorithm.
//!
//! **O(N)**

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
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::default::Default,

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
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::default::Default,
{
    //_radix_sort_impl(array, max, false, signed)
}

fn _radix_sort_impl<T, Max>(array: &mut [T], digits_max: usize, radix: usize)
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::default::Default,
{
   let mut counts = vec![0; radix];
   let mut temp: std::vec::Vec<T> = vec![std::default::Default::default(); array.len()];
}

//! radix sort algorithm.
//!
//! **O(wN)**: w=length of key

use std::convert::{TryFrom, TryInto};

/// Sort in ascending order using a radix sort algorithm.
///
/// The parameter 'radix' is ​​the base on which to sort.
/// If you want decimal based sorting, you can pass 10.
///
/// ```rust
/// use buldak::radix;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, -33, 234, 21, 13];
/// radix::sort(&mut nums, 10);
/// assert_eq!(nums, [-33, 1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T], radix: usize) -> Result<(), String>
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    _radix_sort_scan_impl(array, radix, true)
}

/// Sort in descending order using a radix algorithm.
///
/// The parameter 'radix' is ​​the base on which to sort.
/// If you want decimal based sorting, you can pass 10.
///
/// ```rust
/// use buldak::radix;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, -33, 234, 21, 13];
/// radix::sort_reverse(&mut nums, 10);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1, -33]);
/// ```
pub fn sort_reverse<T>(array: &mut [T], radix: usize) -> Result<(), String>
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    _radix_sort_scan_impl(array, radix, false)
}

fn _radix_sort_impl<T>(
    array: &mut [T],
    digits_max: usize,
    radix: usize,
    asc: bool,
) -> Result<(), String>
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
            for i in (0_isize..(counter.len() as isize)).rev() {
                while let Some(value) = counter[i as usize].pop_front() {
                    array[pos] = value;
                    pos += 1;
                }
            }
            for i in (0_isize..(neg_counter.len() as isize)).rev() {
                while let Some(value) = neg_counter[i as usize].pop_back() {
                    array[pos] = value;
                    pos += 1;
                }
            }
        }
    }

    Ok(())
}

fn _radix_sort_scan_impl<T>(array: &mut [T], radix: usize, asc: bool) -> Result<(), String>
where
    T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
    <T as TryInto<isize>>::Error: std::fmt::Debug,
{
    if array.len() == 0 {
        return Ok(());
    }

    let mut abs_max: isize = array[0].to_owned().try_into().unwrap().abs();
    for e in array.iter() {
        let e: isize = e.to_owned().try_into().unwrap().abs();
        if e > abs_max {
            abs_max = e;
        }
    }

    let abs_max = abs_max as f64;

    let digits_max = abs_max.log(radix as f64) as usize + 1;

    return _radix_sort_impl(array, digits_max, radix, asc);
}

// /// Sort in ascending order using a radix sort algorithm.
// ///
// /// The parameter 'radix' is ​​the base on which to sort.
// /// If you want decimal based sorting, you can pass 10.
// ///
// /// The parameter 'digits_max' is the maximum number of digits in the array.
// /// For example, if the maximum number in the array does not exceed 9999, you can pass 4.
// /// Any value beyond this number will cause an error.
// ///
// /// ```rust
// /// use buldak::radix;
// ///
// /// let mut nums = [1, 4, 2, 3, 5, 111, -33, 234, 21, 13];
// /// radix::sort(&mut nums, 10, 4);
// /// assert_eq!(nums, [-33, 1, 2, 3, 4, 5, 13, 21, 111, 234]);
// /// ```
// pub fn sort<T>(array: &mut [T], radix: usize, digits_max: usize)-> Result<(), String>
// where
//     T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
//     <T as TryInto<isize>>::Error: std::fmt::Debug,
// {
//     _radix_sort_impl(array, digits_max, radix, true)
// }

// /// Sort in descending order using a radix algorithm.
// ///
// /// The parameter 'radix' is ​​the base on which to sort.
// /// If you want decimal based sorting, you can pass 10.
// ///
// /// The parameter 'digits_max' is the maximum number of digits in the array.
// /// For example, if the maximum number in the array does not exceed 9999, you can pass 4.
// /// Any value beyond this number will cause an error.
// ///
// /// ```rust
// /// use buldak::radix;
// ///
// /// let mut nums = [1, 4, 2, 3, 5, 111, -33, 234, 21, 13];
// /// radix::sort_reverse(&mut nums, 10, 4);
// /// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1, -33]);
// /// ```
// pub fn sort_reverse<T>(array: &mut [T], radix: usize, digits_max: usize) -> Result<(), String>
// where
//     T: TryInto<isize> + TryFrom<isize> + std::clone::Clone,
//     <T as TryInto<isize>>::Error: std::fmt::Debug,
// {
//     _radix_sort_impl(array, digits_max, radix, false)
// }

mod tests {
    #[test]
    fn sort_ascending() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = vec![TestCase {
            input: vec![1, 4, 2, 3, 5, 111, 234, 21, 13],
            expected: vec![1, 2, 3, 4, 5, 13, 21, 111, 234],
        }];

        for case in test_cases {
            let mut actual = case.input.clone();
            super::sort(&mut actual, 10).unwrap();
            assert_eq!(actual, case.expected);
        }
    }

    #[test]
    fn sort_descending() {
        struct TestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = vec![TestCase {
            input: vec![1, 4, 2, 3, 5, 111, 234, 21, 13],
            expected: vec![234, 111, 21, 13, 5, 4, 3, 2, 1],
        }];

        for case in test_cases {
            let mut actual = case.input.clone();
            super::sort_reverse(&mut actual, 10).unwrap();
            assert_eq!(actual, case.expected);
        }
    }
}

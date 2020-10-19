//! counting sort algorithm.
//!
//! **O(N)**

fn _counting_impl<T, Max>(array: &mut [T], max: Max, asc: bool, signed: bool) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
    Max: std::convert::TryInto<isize>,
    <Max as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    let max: isize = max
        .try_into()
        .expect("Conversion to isize failed. the max value must be convertible to isize.");
    let max = if max >= 0 { max } else { max * -1 };

    let mut pos_count = vec![0; max as usize];
    let mut neg_count = if signed {
        vec![0; max as usize]
    } else {
        vec![]
    }; // except 0

    let mut i = 0;

    while i < array.len() {
        let e: isize = array[i].clone().try_into().expect(
            "Conversion to isize failed. All elements of the array must be convertible to isize.",
        );

        if max < (if e < 0 { e * -1 } else { e }) {
            return std::result::Result::Err(
                "The maximum range passed by max was exceeded.".to_string(),
            );
        }

        if e >= 0 {
            let count_index = e as usize;
            pos_count[count_index] += 1;
        } else if signed {
            let count_index = (e * -1 + 1) as usize;
            neg_count[count_index] += 1;
        } else {
            return std::result::Result::Err("Now this Function can only receive natural numbers, but they contain negative numbers.".to_string());
        }

        i += 1;
    }

    let mut total_index = 0;

    if asc {
        if signed {
            for i in (0..neg_count.len()).rev() {
                for _ in 0..neg_count[i] {
                    let value: T = T::try_from(i as isize * -1 - 1).unwrap();
                    array[total_index] = value;

                    total_index += 1;
                }
            }
        }

        for i in 0..pos_count.len() {
            for _ in 0..pos_count[i] {
                let value: T = T::try_from(i as isize).unwrap();
                array[total_index] = value;

                total_index += 1;
            }
        }
    } else {
        for i in (0..pos_count.len()).rev() {
            for _ in 0..pos_count[i] {
                let value: T = T::try_from(i as isize).unwrap();
                array[total_index] = value;

                total_index += 1;
            }
        }

        if signed {
            for i in 0..neg_count.len() {
                for _ in 0..neg_count[i] {
                    let value: T = T::try_from(i as isize * -1 - 1).unwrap();
                    array[total_index] = value;

                    total_index += 1;
                }
            }
        }
    };

    return Ok(());
}

/// Sort in ascending order using a counting sort algorithm.
///
/// The parameter 'max' is the absolute maximum value of the received array.
/// Any elements beyond this value will result in an error.
///
/// The parameter 'signed' chooses whether to support negative numbers.
/// When using only natural numbers, it is twice as efficient to set signed to false.
/// If signed is set to false and the array contains negative elements, an error occurs.
///
/// ```rust
/// use buldak::counting;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// counting::sort(&mut nums, 300, true);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T, Max>(array: &mut [T], max: Max, signed: bool) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
    Max: std::convert::TryInto<isize>,
    <Max as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    _counting_impl(array, max, true, signed)
}

/// Sort in descending order using a counting algorithm.
///
/// The parameter 'max' is the absolute maximum value of the received array.
/// Any elements beyond this value will result in an error.
///
/// The parameter 'signed' chooses whether to support negative numbers.
/// When using only natural numbers, it is twice as efficient to set signed to false.
/// If signed is set to false and the array contains negative elements, an error occurs.
///
/// ```compile_fail
/// use buldak::counting;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// counting::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T, Max>(array: &mut [T], max: Max, signed: bool) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
    Max: std::convert::TryInto<isize>,
    <Max as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    _counting_impl(array, max, false, signed)
}

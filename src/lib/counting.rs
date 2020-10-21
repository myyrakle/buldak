//! counting sort algorithm.
//!
//! **O(N)**

/// Sort in ascending order using a counting sort algorithm.
///
/// ```rust
/// use buldak::counting;
///
/// let mut nums = [1, 4, 2, 3, 5, -44, 111, 234, 21, 13];
/// counting::sort(&mut nums);
/// assert_eq!(nums, [-44, 1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    _counting_scan_impl(array, true, true)
}

// pub fn sort<T, Max>(array: &mut [T], max: Max, signed: bool) -> Result<(), String>
// where
//     T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
//     <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
//     <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
//     Max: std::convert::TryInto<isize>,
//     <Max as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
// {
//     _counting_impl(array, max, true, signed)
// }

/// Sort in descending order using a counting algorithm.
///
/// ```rust
/// use buldak::counting;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13, -2];
/// counting::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1, -2]);
/// ```
pub fn sort_reverse<T>(array: &mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    _counting_scan_impl(array, false, true)
}

// pub fn sort_reverse<T, Max>(array: &mut [T], max: Max, signed: bool) -> Result<(), String>
// where
//     T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
//     <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
//     <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
//     Max: std::convert::TryInto<isize>,
//     <Max as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
// {
//     _counting_impl(array, max, false, signed)
// }

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

    let mut pos_count = vec![0; max as usize + 1];
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
            let count_index = (e * -1 - 1) as usize;
            neg_count[count_index] += 1;
        } else {
            return Err("Now this Function can only receive natural numbers, but they contain negative numbers.".to_string());
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

fn _counting_scan_impl<T>(array: &mut [T], asc: bool, signed: bool) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    if array.len() == 0 {
        return Ok(());
    } 

    let mut abs_max:isize = array[0].to_owned().try_into().unwrap().abs();
    for e in array.iter() {
        let e:isize = e.to_owned().try_into().unwrap().abs();
        if e > abs_max {
            abs_max=e;
        }
    }

    return _counting_impl(array, abs_max, asc, signed);
}
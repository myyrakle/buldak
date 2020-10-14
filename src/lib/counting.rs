mod utils;

pub fn sort_core<T, Hint>(array: &mut [T], max: Hint, asc: bool, signed: bool)
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
    Hint: std::convert::TryInto<isize>,
    <Hint as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    let max: isize = max
        .try_into()
        .expect("Conversion to isize failed. the max value must be convertible to isize.");
    let max: usize = if max >= 0 { max } else { max * -1 } as usize;

    let mut pos_count = vec![0; max];
    let mut neg_count = if signed { vec![0; max] } else { vec![] }; // except 0

    let mut i = 0;

    while i < array.len() {
        let e: isize = array[i].clone().try_into().expect(
            "Conversion to isize failed. All elements of the array must be convertible to isize.",
        );

        if e >= 0 {
            let count_index = e as usize;
            pos_count[count_index] += 1;
        } else if signed {
            let count_index = (e * -1 + 1) as usize;
            neg_count[count_index] += 1;
        } else {
            panic!("Now this Function can only receive natural numbers, but they contain negative numbers.");
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
}

/**
Sort in ascending order using a counting sort algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
counting::sort(&mut nums);
assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
```
*/
pub fn sort_hint<T, Hint>(array: &mut [T], max: Hint, asc: bool, signed: bool)
where
    T: std::convert::TryInto<isize> + std::clone::Clone,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    Hint: std::convert::TryInto<isize>,
    <Hint as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
{
    let max: isize = max
        .try_into()
        .expect("Conversion to isize failed. the max value must be convertible to isize.");
    let max: usize = if max >= 0 { max } else { max * -1 } as usize;

    let mut pos_count = vec![0; max];
    let mut neg_count = vec![0; max]; // except 0

    let mut i = 0;

    while i < array.len() {
        let e: isize = array[i].clone().try_into().expect(
            "Conversion to isize failed. All elements of the array must be convertible to isize.",
        );

        if e >= 0 {
            let count_index = e as usize;
            pos_count[count_index] += 1;
        } else {
            let count_index = (e - 1) as usize;
            neg_count[count_index] += 1;
        }

        i += 1;
    }
}

/** Sort in descending order using a bubble counting algorithm.

```
let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
counting::sort_reverse(&mut nums);
assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
```
*/
pub fn sort_reverse<T>(arr: &mut [T])
where
    T: std::cmp::Ord,
{
}

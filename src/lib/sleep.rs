//! sleep sort algorithm.
//!
//! unstable sort  
//! **O(N)**

/// Sort in ascending order using a sleep sort algorithm.
///
/// ```rust
/// use buldak::sleep;
///
/// let mut nums = [1, 4, 2, 3, 5, -44, 111, 234, 21, 13];
/// sleep::sort(&mut nums);
/// assert_eq!(nums, [-44, 1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: & mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::marker::Sync,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    _sleep_sort_impl(array, true)
}


/// Sort in descending order using a sleep algorithm.
///
/// ```rust
/// use buldak::sleep;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13, -2];
/// sleep::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1, -2]);
/// ```
pub fn sort_reverse<T>(array: & mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::marker::Sync,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    _sleep_sort_impl(array, false)
}



fn _sleep_sort_impl<T>(
    array: &mut [T],
    asc: bool
) -> Result<(), String>
where
    T: std::convert::TryInto<isize> + std::convert::TryFrom<isize> + std::clone::Clone + std::marker::Sync,
    <T as std::convert::TryInto<isize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<isize>>::Error: std::fmt::Debug,
{
    use std::{thread, time};
    use std::sync::{Arc, Mutex};

    let original = Arc::new(Mutex::new(array.to_owned()));

    let mut shared = Arc::new(Mutex::new(vec![]));
    let mut handlers = vec![];

    for e in original.lock().unwrap().iter() {
        let mut data = Arc::clone(&shared);

        let n: isize = e.to_owned().try_into().unwrap();

        handlers.push(thread::spawn(move ||{
            thread::sleep(time::Duration::from_millis(n as u64));
            data.lock().unwrap().push(e);
        }));
    }

    for handler in handlers {
        handler.join();
    }
/*
    let result = shared.lock().unwrap();

    if asc {
        for i in 0..array.len() {
            array[i] = result[i].clone();
        }
    }
    else {
        let len = array.len();
        for i in 0..array.len() {
            array[i] = result[len-i-1].clone();
        }
    }
*/

    return Ok(());
}


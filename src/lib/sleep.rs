//! sleep sort algorithm.
//!
//! unstable sort  
//! **O(N)**

/// Sort in ascending order using a sleep sort algorithm.
///
/// ```rust
/// use buldak::sleep;
///
/// let mut nums = [6, 4, 2, 3, 1, 5];
/// sleep::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 6]);
/// ```
pub fn sort<T>(array: & mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<usize> + std::convert::TryFrom<usize> + std::clone::Clone + std::marker::Sync + std::marker::Send+ 'static,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    _sleep_sort_impl(array, true)
}


/// Sort in descending order using a sleep algorithm.
///
/// ```rust
/// use buldak::sleep;
///
/// let mut nums = [6, 4, 2, 3, 1, 5];
/// sleep::sort_reverse(&mut nums);
/// assert_eq!(nums, [6, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: & mut [T]) -> Result<(), String>
where
    T: std::convert::TryInto<usize> + std::convert::TryFrom<usize> + std::clone::Clone + std::marker::Sync + std::marker::Send + 'static,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    _sleep_sort_impl(array, false)
}


fn _sleep_sort_impl<T>(
    array: &mut [T],
    asc: bool
) -> Result<(), String>
where
    T: std::convert::TryInto<usize> + std::convert::TryFrom<usize> + std::clone::Clone + std::marker::Sync  + std::marker::Send + 'static,
    <T as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
    <T as std::convert::TryFrom<usize>>::Error: std::fmt::Debug,
{
    use std::{thread, time};
    use std::sync::{Arc, Mutex};

    let original = Arc::new(Mutex::new(array.to_owned()));

    let shared = Arc::new(Mutex::new(vec![]));
    let mut handlers = vec![];

    for e in original.lock().unwrap().iter().cloned() {
        let data = Arc::clone(&shared);

        let n: usize = e.to_owned().try_into().unwrap();

        handlers.push(thread::spawn(move ||{
            thread::sleep(time::Duration::from_secs(n as u64));
            data.lock().unwrap().push(e);
        }));
    }

    for handler in handlers {
        handler.join().unwrap();
    }

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

    return Ok(());
}


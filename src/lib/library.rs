//! quick sort algorithm.
//!
//! unstable sort  
//! **average:O(Nlog₂N), worst:O(N²)**

//mod utils;

/// Sort in ascending order using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a quick sort algorithm.
///
/// ```rust
/// use buldak::quick;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// quick::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _quick_sort_impl(array, compare)
}

fn _library_sort<T, F>(array: &mut [T], n: usize, epsilon: f64, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let epsilon = 1;

    let s_len = ((1+epsilon) * array.len() as f64) as usize;
    let mut buffer:Vec<Option<T>> = vec![None; s_len];

    let mut i = 0;
    let mut j = 0;

    _library_sort_impl

    while i<s_len && j <array.len() {
        if buffer[i].is_some() {
            array[j] = buffer[i].unwrap().to_owned();
            j+=1;
        }
    }
}

fn _library_sort_impl<T, F>(array: &mut [T], n: usize, epsilon: f64, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let mut goal = 1;
    let mut pos = 0;

    let sLen = ((1+epsilon)*n as i64) as usize;

    while pos < n {
        for i in 1..=goal {

        }

        
        goal *= 2;
    }
}

fn _rebalance<T, F>(array: &mut [T], n: usize, epsilon: f64, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
}

//! tim sort algorithm.
//!
//! **O(Nlog₂N)**

// not impl

use std::default::Default;

/// Sort in ascending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone + std::default::Default,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone + std::default::Default,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a tim sort algorithm.
///
/// ```rust
/// use buldak::tim;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// tim::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone + std::default::Default,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _tim_sort_impl(array, compare)
}

const RUN:usize = 32;

// Iterative Timsort function to sort the 
// array[0...n-1] (similar to merge sort)
fn _tim_sort_impl<T, F>(array: &mut [T], compare: F) 
where
    T: std::cmp::Ord + std::clone::Clone + std::default::Default,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    // Sort individual subarrays of size RUN 
    let mut i = 0;
    while i<array.len() {
        _insertion_sort(array, i, std::cmp::min(i+31, array.len()-1), compare.clone());
        i+=RUN;
    }

    // Start merging from size RUN (or 32).  
    // It will merge 
    // to form size 64, then 128, 256  
    // and so on .... 
    let mut size = RUN;
    while size<array.len() {
        let mut left = 0;

        // pick starting point of  
        // left sub array. We 
        // are going to merge  
        // array[left..left+size-1] 
        // and array[left+size, left+2*size-1] 
        // After every merge, we  
        // increase left by 2*size 
        while left < array.len() {

            // find ending point of  
            // left sub array 
            // mid+1 is starting point  
            // of right sub array 
            let middle = left + size -1 ;
            let right = std::cmp::min(left + 2*size -1, array.len()-1);

            // merge sub array arr[left.....mid] & 
            // arr[mid+1....right] 
            _merge(array, left, middle, right, compare.clone());

            left += size*2;
        }

        size*=2;
    }
}

// Merge function merges the sorted runs 
fn _merge<T, F>(
    array: &mut [T],
    left: usize,
    middle: usize,
    right: usize,
    compare: F,
) where
    T: std::cmp::Ord + std::clone::Clone + std::default::Default,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    // Original array is broken in two parts 
    // left and right array 
    let left_len = middle - left + 1;
    let right_len = right - middle;

    let mut array_left:Vec<T> = vec![Default::default(); left_len];
    let mut array_right:Vec<T> = vec![Default::default(); right_len];

    for i in 0..left_len {
        array_left[i] = array[left+i].clone();
    }

    for i in 0..right_len {
        array_right[i] = array[left+i+middle].clone();
    }

    let mut i = 0; //left array index
    let mut j = 0; //right array index
    let mut k = 1; //full array index

    // After comparing, we  
    // merge those two array 
    // in larger sub array 
    while i<left_len && j<right_len {
        if compare(&array_left[i], &array_right[j]) == std::cmp::Ordering::Greater {
            array[k] = array_right[j].clone();
            j+=1;
        } else {
            array[k] = array_left[i].clone();
            i+=1;
        }
        k+=1;
    }

    // Copy remaining elements of left, if any 
    while i < left_len {
        array[k] = array_left[i].clone();
        k+=1;
        i+=1;
    }

    // Copy remaining element of right, if any 
    while j < right_len {
        array[k] = array_right[j].clone();
        k+=1;
        j+=1;
    }
}

// This function sorts array from left index to 
// to right index which is of size atmost RUN
fn _insertion_sort<T, F>(array: &mut [T], left: usize, right: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in (left+1)..=right {
        let temp = array[i].clone();
        let mut j = (i - 1) as isize;

        while j>=left as isize && compare(&array[j as usize], &temp) == std::cmp::Ordering::Greater {
            array[(j+1) as usize] = array[j as usize].clone();
            j-=1;
        }
        array[(j+1) as usize] = temp;
    }
}
//! intro sort algorithm.
//!
//! unstable sort  
//! **O(Nlog₂N)**

// not impl

mod utils;

/// Sort in ascending order using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort(&mut nums);
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r))
}

/// Sort in descending order using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort_reverse(&mut nums);
/// assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
/// ```
pub fn sort_reverse<T>(array: &mut [T])
where
    T: std::cmp::Ord + std::clone::Clone,
{
    sort_by(array, |l, r| l.cmp(r).reverse())
}

/// It takes a comparator function to determine the order,
/// and sorts it using a intro sort algorithm.
///
/// ```rust
/// use buldak::intro;
///
/// let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
/// intro::sort_by(&mut nums, |l, r| l.cmp(r));
/// assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
/// ```
pub fn sort_by<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _intro_sort_impl(array, compare)
}

fn _intro_sort_impl<T, F>(array: &mut [T], compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let max_depth = (array.len() as f64).log2().floor() as isize * 2;
    _intro_sort_recursive(array, 0, array.len() - 1, max_depth, compare)
}

fn _intro_sort_recursive<T, F>(
    array: &mut [T],
    begin: usize,
    end: usize,
    mut max_depth: isize,
    compare: F,
) where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if end - begin > 16 {
        if max_depth == 0 {
            _heap_sort(array, begin, end, compare);
            return ();
        }

        max_depth -= 1;
        let pivot = _find_pivot(
            array,
            begin,
            begin + ((end - begin) / 2) + 1,
            end,
            compare.clone(),
        );
        utils::swap(array, pivot, end);

        let pivot = _intro_partition(array, begin, end, compare.clone());
        _intro_sort_recursive(array, begin, pivot - 1, max_depth, compare.clone());
        _intro_sort_recursive(array, pivot + 1, end, max_depth, compare);
    } else {
        _insertion_sort(array, begin, end, compare);
    }
}

fn _max_index<T, F>(array: &[T], left: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if compare(&array[left], &array[right]) == std::cmp::Ordering::Less {
        left
    } else {
        right
    }
}

fn _min_index<T, F>(array: &[T], left: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    if compare(&array[left], &array[right]) != std::cmp::Ordering::Less {
        left
    } else {
        right
    }
}

fn _find_pivot<T, F>(array: &[T], left: usize, middle: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    let max = _max_index(
        array,
        _max_index(array, left, right, compare.clone()),
        middle,
        compare.clone(),
    );
    let min = _min_index(
        array,
        _min_index(array, left, right, compare.clone()),
        middle,
        compare.clone(),
    );

    if left != max && left != min {
        return left;
    }

    if right != max && right != min {
        return right;
    }
    return middle;
}

fn _intro_partition<T, F>(array: &mut [T], left: usize, right: usize, compare: F) -> usize
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let pivot = array[right].clone();

    let mut i = left - 1;

    for j in left..=(right - 1) {
        if compare(&array[j], &pivot) != std::cmp::Ordering::Greater {
            i += 1;
            utils::swap(array, i, j);
        }
    }
    utils::swap(array, i + 1, right);
    return i + 1;
}

fn _heap_sort<T, F>(array: &mut [T], begin: usize, end: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    _make_heap(array, begin, end, compare.clone());
    for i in (begin..end).rev() {
        utils::swap(array, begin, i);
        _make_heap(array, begin, i, compare.clone());
    }
}

fn _make_heap<T, F>(array: &mut [T], begin: usize, end: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in (begin + 1)..(end) {
        let mut child = i;

        while child > begin {
            let root = (child - 1) / 2;
            if compare(&array[root], &array[child]) == std::cmp::Ordering::Less {
                utils::swap(array, root, child);
            }
            child = root;
        }
    }
}

fn _insertion_sort<T, F>(array: &mut [T], left: usize, right: usize, compare: F)
where
    T: std::cmp::Ord + std::clone::Clone,
    F: Fn(&T, &T) -> std::cmp::Ordering + std::clone::Clone,
{
    for i in (left + 1)..=right {
        let temp = array[i].clone();
        let mut j = (i - 1) as isize;

        while j >= left as isize
            && compare(&array[j as usize], &temp) == std::cmp::Ordering::Greater
        {
            array[(j + 1) as usize] = array[j as usize].clone();
            j -= 1;
        }
        array[(j + 1) as usize] = temp;
    }
}

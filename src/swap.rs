use std::cmp::Ordering;
use std::mem;

fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        Ordering::Equal => return,

        // get the smallest and largest of the two indices
        Ordering::Less => (i, j),
        Ordering::Greater => (j, i),
    };

    let (init, tail) = x.split_at_mut(hi);
    mem::swap(&mut init[lo], &mut tail[0]);
}

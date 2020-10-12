pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = vec![1, 4, 2, 3, 5, 111, 234, 21, 13];
    bubble_sort_reverse(&mut nums);
    //println!("{:?}", 1.cmp(&4));
    // bubble_sort_by(&mut nums, |l, r| match l.cmp(r) {
    //     std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
    //     std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
    //     _ => std::cmp::Ordering::Equal,
    // });
    println!("{:?}", nums);
}

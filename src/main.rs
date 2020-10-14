pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
    bubble::sort_by(&mut nums, |l, r| l.cmp(r));
    //assert_eq!(nums, [234, 111, 21, 13, 5, 4, 3, 2, 1]);
    println!("{:?}", nums);
}

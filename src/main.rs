pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
    stooge::sort(&mut nums);
    assert_eq!(nums, [1, 2, 3, 4, 5, 13, 21, 111, 234]);
}

pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = [1, 4, 2, 3, 5, 111, 234, 21, 13];
    radix::sort(&mut nums, 10);
    println!("{:?}", nums);
}

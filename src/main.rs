pub mod lib;
pub use lib::*;


fn main() {
    let mut nums = [1112, 343, 3, 33, 5, 111, 234, 21, 13];
    intro::sort_reverse(&mut nums);
    println!("{:?}", nums);
}

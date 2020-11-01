pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = [1, 1, 1, 1, 1, 1];
    oddeven::sort_reverse(&mut nums);
    //bitonic::sort(&mut nums);
    println!("{:?}", nums);
}

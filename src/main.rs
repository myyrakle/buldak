pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = vec![1, 4, 2, 3, 5, 11, 23, 21, 0];
    //let mut re:&'static mut Vec<_> = &mut nums;
    bitonic::sort_reverse(&mut nums).unwrap();
    //bitonic::sort(&mut nums);
    println!("{:?}", nums);
}

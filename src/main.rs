pub mod lib;
pub use lib::*;

fn main() {
    let mut nums = vec![5,4,1,3,2,7,8,2];
    let mut re:&'static mut Vec<_> = &mut nums;
    sleep::sort_reverse(re);
    //bitonic::sort(&mut nums);
    println!("{:?}", nums);
}

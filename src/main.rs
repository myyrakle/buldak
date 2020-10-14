pub mod lib;
pub use lib::*;

fn main() {
    let mut v = vec![1, 4, 2, 3, 5, 111, 234, 21, 13];
    counting::sort(&mut v, 1000, false).unwrap();

    println!("{:?}", v);
}

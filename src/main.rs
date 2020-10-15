pub mod lib;
pub use lib::*;

fn main() {
    let mut v = vec![5, 2, 3, 4, 1];

    stooge::sort_reverse(&mut v);
    println!("{:?}", v);
}

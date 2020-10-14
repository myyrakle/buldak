pub mod lib;
pub use lib::*;

fn first<N>(n: N) -> usize
where
    N: std::convert::TryInto<usize>,
    <N as std::convert::TryInto<usize>>::Error: std::fmt::Debug,
{
    let i: usize = n.try_into().unwrap();
    let mut nums = vec![1, 4, 2, 3, 5, 111, 234, 21, 13];
    nums[i]
}

fn main() {
    let v = vec![0; 10];

    for e in v.iter().rev() {
        println!("{}", e);
    }
}

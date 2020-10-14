/*!
It is a library that provides various sorting functions.

## install
If cargo-edit is installed, you can install it like this:
```
cargo add buldak
```
If not, you have to manually add the dependency to Cargo.toml.
```
[dependencies]
buldak = "*"
```

## use
If you have performed the installation process well,
you can sort by passing the values ​​in an array format as follows.
```
use buldak::*;

fn main()
{
    let mut nums = [6, 34, 3, 1, 2];
    bubble::sort(&mut nums);
    println!("{:?}", nums);
}
```

## features
- bubble sort
- selection sort
- double selection sort
- counting sort
- ... more later

## link
- [document](https://docs.rs/buldak)
- [repository](https://github.com/myyrakle/buldak)
*/

#[path = "lib/bubble.rs"]
pub mod bubble;

#[path = "lib/selection.rs"]
pub mod selection;

#[path = "lib/double_selection.rs"]
pub mod double_selection;

#[path = "lib/counting.rs"]
pub mod counting;

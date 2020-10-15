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
- smart bubble sort
- cocktail shaker sort
- selection sort
- double selection sort
- insertion sort
- stooge sort
- quick sort
- merge sort
- heap sort
- counting sort
- bogo sort
- ... more later

## link
- [document](https://docs.rs/buldak)
- [repository](https://github.com/myyrakle/buldak)
*/

#[path = "lib/bubble.rs"]
pub mod bubble;

#[path = "lib/smart_bubble.rs"]
pub mod smart_bubble;

#[path = "lib/cocktail_shaker.rs"]
pub mod cocktail_shaker;

#[path = "lib/selection.rs"]
pub mod selection;

#[path = "lib/insertion.rs"]
pub mod insertion;

#[path = "lib/double_selection.rs"]
pub mod double_selection;

#[path = "lib/stooge.rs"]
pub mod stooge;

#[path = "lib/quick.rs"]
pub mod quick;

#[path = "lib/merge.rs"]
pub mod merge;

#[path = "lib/heap.rs"]
pub mod heap;

#[path = "lib/counting.rs"]
pub mod counting;

#[path = "lib/bogo.rs"]
pub mod bogo;

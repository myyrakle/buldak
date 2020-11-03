/*!
It is a library that provides various sorting functions.

## install
If cargo-edit is installed, you can install it like this:
```sh
cargo add buldak
```
If not, you have to manually add the dependency to Cargo.toml.
```toml
[dependencies]
buldak = "0.24.0"
```

## use
If you have performed the installation process well,
you can sort by passing the values in an array format as follows.
```rust
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
- binary insertion sort
- stooge sort
- gnome sort
- comb sort
- cycle sort
- oddeven sort
- quick sort
- merge sort
- heap sort
- intro sort
- tim sort
- counting sort
- radix sort
- shell sort
- bogo sort
- sleep sort
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

#[path = "lib/binary_insertion.rs"]
pub mod binary_insertion;

#[path = "lib/double_selection.rs"]
pub mod double_selection;

#[path = "lib/shell.rs"]
pub mod shell;

#[path = "lib/stooge.rs"]
pub mod stooge;

#[path = "lib/gnome.rs"]
pub mod gnome;

#[path = "lib/comb.rs"]
pub mod comb;

#[path = "lib/cycle.rs"]
pub mod cycle;

#[path = "lib/oddeven.rs"]
pub mod oddeven;

/*#[path = "lib/bitonic.rs"]
pub mod bitonic;*/

#[path = "lib/quick.rs"]
pub mod quick;

#[path = "lib/merge.rs"]
pub mod merge;

#[path = "lib/heap.rs"]
pub mod heap;

#[path = "lib/intro.rs"]
pub mod intro;

#[path = "lib/tim.rs"]
pub mod tim;

#[path = "lib/counting.rs"]
pub mod counting;

#[path = "lib/radix.rs"]
pub mod radix;

#[path = "lib/bogo.rs"]
pub mod bogo;

#[path = "lib/sleep.rs"]
pub mod sleep;

#[path = "lib/stalin.rs"]
pub mod stalin;

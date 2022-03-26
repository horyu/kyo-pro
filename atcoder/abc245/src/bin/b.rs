#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut hs: BTreeSet<_> = (0..=2000).collect();
    for a in aa {
        hs.remove(&a);
    }
    let rs = hs.iter().min().unwrap();
    println!("{rs}");
}

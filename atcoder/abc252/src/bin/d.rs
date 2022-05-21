#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hm = HashMap::new();
    for &a in &aa {
        *hm.entry(a).or_insert(0) += 1usize;
    }
    let mut rs = n * (n - 1) * (n - 2) / 6;
    for &c in hm.values() {
        rs -= c * c.saturating_sub(1) / 2 * (n - c)
            + c * c.saturating_sub(1) * c.saturating_sub(2) / 6;
    }
    println!("{rs}");
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut hh: [usize; n]
    };
    hh.sort();
    let min = (0usize..=(n - k))
        .map(|left| hh[left + k - 1] - hh[left])
        .min()
        .unwrap();
    println!("{}", min);
}

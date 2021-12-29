#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        aa: [isize; 3]
    };
    let mut aa: Vec<_> = aa
        .iter()
        .combinations(2)
        .map(|arr| (arr[0] - arr[1]).abs())
        .collect();
    aa.sort_unstable();
    println!("{}", aa[0] + aa[1]);
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    let rs = (1..n).fold(0usize, |acc, k| acc + aa[n - 1 - k / 2]);
    println!("{}", rs)
}

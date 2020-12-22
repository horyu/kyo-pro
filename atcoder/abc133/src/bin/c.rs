#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    use std::cmp::min;
    let mut rs = std::usize::MAX;
    for i in l..=min(r - 1, l + 2019 - 1) {
        for j in (i + 1)..=min(r, l + 2019) {
            rs = min(rs, i * j % 2019)
        }
    }
    println!("{}", rs);
}

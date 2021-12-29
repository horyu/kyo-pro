#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut dd: [usize; n]
    };
    dd.sort_unstable();
    println!("{}", dd[n / 2] - dd[n / 2 - 1]);
}

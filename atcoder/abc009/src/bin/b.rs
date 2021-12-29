#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    aa.dedup();
    println!("{}", aa[aa.len() - 2]);
}

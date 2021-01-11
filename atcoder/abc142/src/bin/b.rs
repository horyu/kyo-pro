#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        hh: [usize; n]
    };
    let rs = hh.iter().filter(|&&h| h >= k).count();
    println!("{}", rs);
}

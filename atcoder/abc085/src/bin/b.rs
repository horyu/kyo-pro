#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut dd: [usize; n]
    };
    dd.sort();
    dd.dedup();
    println!("{}", dd.len());
}

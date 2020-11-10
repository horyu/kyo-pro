#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        pp: [usize; n]
    };
    let rs = pp.iter().sum::<usize>() - pp.iter().max().unwrap() / 2;
    println!("{}", rs);
}

#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut vv: Vec<(usize, usize)> = aa.into_iter().enumerate().collect();
    vv.sort_by_key(|&(_i, a)| a);
    println!("{}", vv.iter().map(|&(i, _a)| i + 1).join(" "));
}

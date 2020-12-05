#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        vv: [usize; n],
        cc: [usize; n]
    };
    let rs = vv
        .iter()
        .zip(cc.iter())
        .fold(0, |acc, (v, &c)| acc + v.checked_sub(c).unwrap_or(0));
    println!("{}", rs);
}

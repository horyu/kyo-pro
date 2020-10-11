#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xx: [usize; n]
    };
    let mut yy = xx.clone();
    yy.sort();
    let border = yy[(n - 1) / 2];
    for x in xx {
        println!("{}", yy[[n / 2, n / 2 - 1][(x > border) as usize]]);
    }
}

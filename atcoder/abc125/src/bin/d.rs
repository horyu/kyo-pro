#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut vv = vec![(0, 0); n + 1];
    vv[0].1 = -500000000;
    for (i, a) in aa[0..n].iter().enumerate() {
        vv[i + 1].0 = std::cmp::max(vv[i].0 + a, vv[i].1 - a);
        vv[i + 1].1 = std::cmp::max(vv[i].0 - a, vv[i].1 + a);
    }
    println!("{}", vv[n].0);
}

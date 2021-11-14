#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let mut vv = vec![std::isize::MAX; n];
    vv[0] = 0;
    vv[1] = (aa[1] - aa[0]).abs();
    for i in 2..n {
        vv[i] = (vv[i - 1] + (aa[i] - aa[i - 1]).abs()).min(vv[i - 2] + (aa[i] - aa[i - 2]).abs());
    }
    println!("{}", vv[n - 1]);
}

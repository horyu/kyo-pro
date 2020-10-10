#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        aa: [usize; m]
    };
    let left = (0..x).filter(|i| aa.contains(i)).collect::<Vec<_>>().len();
    let right = (x..n).filter(|i| aa.contains(i)).collect::<Vec<_>>().len();
    println!("{}", std::cmp::min(left, right));
}

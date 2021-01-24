#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut bb: Vec<usize> = aa.clone();
    bb.sort_unstable();
    bb.dedup();
    let mut max = 0;
    for x in bb {
        let tmp = aa.split(|&a| a < x).map(|arr| x * arr.len()).max().unwrap();
        max = max.max(tmp);
    }
    println!("{}", max);
}

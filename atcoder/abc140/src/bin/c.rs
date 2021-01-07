#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        bb: [usize; n - 1]
    };
    let mut aa = vec![0; n];
    aa[0] = bb[0];
    for i in 1..(n - 1) {
        aa[i] = bb[i - 1].min(bb[i]);
    }
    aa[n - 1] = bb[n - 2];
    println!("{}", aa.iter().sum::<usize>());
    dbg!(aa);
}

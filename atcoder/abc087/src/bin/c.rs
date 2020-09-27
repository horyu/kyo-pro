#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa1: [usize; n],
        aa2: [usize; n],
    };
    let mut max = 0;
    for i in 0..n {
        let sum: usize = aa1[0..=i].iter().sum::<usize>() + aa2[i..n].iter().sum::<usize>();
        max = std::cmp::max(max, sum);
    }
    println!("{}", max);
}

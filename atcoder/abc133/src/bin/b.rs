#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        d: usize,
        xxx: [[isize; d]; n]
    };
    let pows: Vec<_> = xxx
        .iter()
        .combinations(2)
        .map(|tmp| {
            let xa = tmp[0];
            let xb = tmp[1];
            (0usize..d).fold(0, |acc, i| acc + (xa[i] - xb[i]).pow(2))
        })
        .collect();
    let rs = pows
        .iter()
        .filter(|&&pow| (0..=pow).any(|i| i * i == pow))
        .count();
    println!("{}", rs);
}

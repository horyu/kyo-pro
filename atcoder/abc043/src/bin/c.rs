#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        aa: [isize; n]
    };
    // f(k)
    // = Σ (k-ai)^2
    // = Σ (k*k -2k*ai + ai*ai)
    // = N*k*k -2k*Σai + Σ(ai*ai)
    // = N * (k*k -2k/N*Σai) + Σ(ai*ai)
    // = N*(k - 1/N*Σai)^2 - 1/N*(Σai)^2 + Σ(ai*ai)
    let sig_ai: isize = aa.iter().sum();
    let mut min = std::isize::MAX;
    for diff in [-1, 0, 1].iter() {
        let k = sig_ai / n + diff;
        let cost = aa.iter().fold(0, |acc, a| acc + (k - a).pow(2));
        min = std::cmp::min(min, cost);
    }
    println!("{}", min);
}

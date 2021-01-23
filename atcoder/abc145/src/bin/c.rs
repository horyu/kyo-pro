#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let sum = (0..n).combinations(2).fold(0.0, |acc, pair| {
        let mae = pair[0];
        let ato = pair[1];
        let m = xxyy[mae];
        let a = xxyy[ato];
        acc + (((m.0 - a.0).pow(2) + (m.1 - a.1).pow(2)) as f64).sqrt()
    });
    println!("{}", sum * 2.0 / (n as f64));
}

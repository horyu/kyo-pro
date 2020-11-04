#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        x0: isize,
        xx: [isize; n]
    };
    let rs = xx
        .iter()
        .map(|x| (x0 - x).abs())
        .fold((x0 - xx[0]).abs(), gcd);
    println!("{}", rs);
}

fn gcd(a: isize, b: isize) -> isize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

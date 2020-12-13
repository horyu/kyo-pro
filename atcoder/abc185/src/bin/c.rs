#![allow(unused_imports)]
// use itertools::Itertools;
use num::{BigUint, One};
use proconio::{input, marker::*};

fn main() {
    input! {
        l: usize
    };
    // (l-1) C 11
    let mut rs = BigUint::one();
    for bunbo in (l - 11)..=(l - 1) {
        rs *= bunbo;
    }
    for bunsi in 2usize..=11 {
        rs /= bunsi;
    }
    println!("{}", rs);
}

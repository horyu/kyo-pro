#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

const MOD: isize = 1_000_000_007;

fn main() {
    input! {
        n: isize,
        m: isize
    };
    let diff = (n - m).abs();
    if diff == 0 {
        println!("{}", product(n) * product(m) * 2 % MOD);
    } else if diff == 1 {
        println!("{}", product(n) * product(m) % MOD);
    } else {
        println!("0");
    }
}

fn product(num: isize) -> isize {
    (1..=num).fold(1, |acc, x| acc * x % MOD)
}

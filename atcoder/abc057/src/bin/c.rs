#![allow(unused_imports)]
// use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let rs = (1..=(n.sqrt() + 1))
        .filter(|i| n % i == 0)
        .map(|i| f(i, n / i))
        .min()
        .unwrap();

    println!("{}", rs);
}

fn f(x: usize, y: usize) -> usize {
    let mut z = x.max(y);
    let mut rs = 0;
    while z > 0 {
        z /= 10;
        rs += 1;
    }
    rs
}

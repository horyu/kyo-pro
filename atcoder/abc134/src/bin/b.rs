#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        d: usize
    };
    let r = 2 * d + 1;
    println!("{}", n / r + ((n % r) != 0) as usize);
}

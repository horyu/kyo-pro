#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut m: usize
    };
    let mut sum = 0;
    let num = std::cmp::min(n, m / 2);
    sum += num;
    m -= 2 * num;
    sum += m / 4;
    println!("{}", sum);
}

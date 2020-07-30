#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        x: isize
    };
    // n*(n+1)/2 >= x
    // n*(n+1) >= 2x
    let n = (1..std::isize::MAX)
        .find(|&i| i * (i + 1) >= 2 * x)
        .unwrap();
    println!("{}", n);
}

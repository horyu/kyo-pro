#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: i64,
        b: i64,
        _c: i64,
        k: i64
    };
    let max = if k <= a {
        k
    } else if k <= a + b {
        a
    } else {
        // 1*a + 0*b + (k-a-b) * (-1)
        a - (k - a - b)
    };
    println!("{}", max);
}

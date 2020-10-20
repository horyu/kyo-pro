#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        d: u32,
        mut n: i64
    };
    if n % 100 == 0 {
        n += 1;
    }
    println!("{}", 100i64.pow(d) * n);
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize
    };
    println!("{}", n - a + b);
}

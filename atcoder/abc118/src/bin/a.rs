#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize
    };
    println!("{}", if b % a == 0 { a + b } else { b - a });
}

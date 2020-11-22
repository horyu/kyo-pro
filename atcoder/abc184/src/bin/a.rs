#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize
    };
    println!("{}", a * d - b * c);
}

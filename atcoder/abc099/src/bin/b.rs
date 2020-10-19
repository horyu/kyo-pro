#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    println!("{}", (1..(b - a)).fold(0, |acc, x| acc + x) - a);
}

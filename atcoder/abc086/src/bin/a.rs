#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    println!("{}", ["Odd", "Even"][(a * b % 2 == 0) as usize]);
}

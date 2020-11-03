#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    println!("{}", ["No", "Yes"][(a * b % 2 == 1) as usize]);
}

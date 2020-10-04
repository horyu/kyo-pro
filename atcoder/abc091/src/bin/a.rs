#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize
    };
    println!("{}", ["No", "Yes"][(c <= a + b) as usize]);
}

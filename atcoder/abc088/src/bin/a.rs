#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: usize
    };
    println!("{}", ["No", "Yes"][(n % 500 <= a) as usize]);
}

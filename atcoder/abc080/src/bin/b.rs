#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: String
    };
    let x: usize = n.parse().unwrap();
    let fx: usize = n
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum();
    println!("{}", ["No", "Yes"][(x % fx == 0) as usize]);
}

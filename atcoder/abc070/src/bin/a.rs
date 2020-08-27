#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: String
    };
    println!(
        "{}",
        ["No", "Yes"][(n == n.chars().rev().collect::<String>()) as usize]
    );
}

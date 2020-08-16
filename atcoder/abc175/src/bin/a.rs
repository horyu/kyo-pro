#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String
    };
    println!(
        "{}",
        s.split("S")
            .collect::<Vec<_>>()
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap()
    );
}

#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: i8,
        b: i8
    };
    println!(
        "{}",
        vec![(a - b).abs(), 10 + a - b, 10 - a + b]
            .iter()
            .min()
            .unwrap()
    );
}

#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    println!(
        "{}",
        aa.iter()
            .combinations(2)
            .map(|tmp| (tmp[0] - tmp[1]).abs())
            .max()
            .unwrap()
    );
}

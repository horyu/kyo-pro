#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        w: usize,
        h: usize,
        x: usize,
        y: usize,
    };
    println!(
        "{} {}",
        (w * h) as f64 / 2.0,
        (((2 * x) == w) && (2 * y) == h) as u8
    );
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: f64,
        b: f64,
    };
    // a * (1 - x) = b
    println!("{}", 100.0 - b * 100.0 / a);
}

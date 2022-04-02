#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        xxyy: [(isize, isize); 3]
    };
    let (x, y) = xxyy
        .into_iter()
        .fold1(|acc, (x, y)| (acc.0 ^ x, acc.1 ^ y))
        .unwrap();
    println!("{x} {y}");
}

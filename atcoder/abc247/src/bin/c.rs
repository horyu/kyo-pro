#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let mut s = "1".to_string();
    for i in 2..=(n) {
        s = format!("{s} {i} {s}");
    }
    println!("{s}");
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    };
    let rs = if x <= a {
        1.0
    } else if x <= b {
        c as f64 / (b - a) as f64
    } else {
        0.0
    };
    println!("{rs}");
}

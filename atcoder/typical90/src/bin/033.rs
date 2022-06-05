#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let rs = if h == 1 || w == 1 {
        h * w
    } else {
        h.div_ceil(2) * w.div_ceil(2)
    };
    println!("{rs}");
}

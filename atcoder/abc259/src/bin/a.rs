#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        _n: usize,
        m: usize,
        x: usize,
        t: usize,
        d: usize,
    };
    let rs = t - d * x.saturating_sub(m);
    println!("{rs}");
}

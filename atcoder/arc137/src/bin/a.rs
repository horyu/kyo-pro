#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        l: usize,
        r: usize,
    };
    let rs = (l..=(l + 100).min(r))
        .map(|l| (l..=r).rev().find(|&i| i.gcd(&l) == 1).unwrap_or(l) - l)
        .max()
        .unwrap_or(1);
    println!("{rs}");
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    };
    let i = pp.iter().position(|&p| p == 1).unwrap();
    let j = (i + 1) % n;

    let rs = if pp[j] == 2 {
        i.min(n + 2 - i)
    } else {
        (j + 1).min(1 + n - j)
    };
    println!("{rs}");
}

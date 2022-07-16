#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let m = 2 * n;
    let mut vv = vec![4; n.div_ceil(4)];
    if n % 4 != 0 {
        vv[0] = n % 4;
    }
    let x = vv.into_iter().join("");
    println!("{m}\n{x}");
}

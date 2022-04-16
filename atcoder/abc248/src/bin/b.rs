#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    };
    let mut rs = 0;
    while a < b {
        rs += 1;
        a *= k;
    }
    println!("{rs}");
}

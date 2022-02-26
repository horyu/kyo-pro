#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        aa: [usize; 10]
    };
    let a = aa[0];
    let a = aa[a];
    let a = aa[a];
    println!("{a}");
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut ss = Vec::new();
    for c in 'A'..='Z' {
        for _ in 0..n {
            ss.push(c);
        }
    }
    println!("{}", ss[x - 1]);
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: String,
        k: usize,
    };
    let mut rs = usize::from_str_radix(&n, 8).unwrap();
    for _ in 0..k {
        let mut vv = vec![];
        while rs != 0 {
            vv.push(rs % 9);
            rs /= 9;
        }
        rs = vv
            .into_iter()
            .rev()
            .fold(0, |acc, v| acc * 8 + if v == 8 { 5 } else { v });
    }
    println!("{rs:o}");
}

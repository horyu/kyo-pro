#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [Usize1; 4 * n - 1]
    };
    let mut vv = vec![0; 4 * n];
    for a in aa {
        vv[a] += 1;
    }
    println!("{}", vv.into_iter().position(|v| v == 3).unwrap() + 1);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    // 調和級数 -1
    let rs = (1..=n).rev().fold(0.0, |acc, k| acc + 1.0 / (k as f64)) * (n as f64) - 1.0;
    println!("{}", rs);
}

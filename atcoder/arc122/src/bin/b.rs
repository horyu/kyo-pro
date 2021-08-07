#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n]
    };
    aa.sort_unstable();
    let aa = aa.into_iter().map(|a| a as f64).collect_vec();
    let x = if n.is_odd() {
        aa[n / 2] / 2.0
    } else {
        (aa[(n - 1) / 2] + aa[n / 2]) / 4.0
    };
    let rs = aa.iter().map(|&a| x + a - a.min(x * 2.0)).sum::<f64>() / n as f64;
    println!("{}", rs);
}

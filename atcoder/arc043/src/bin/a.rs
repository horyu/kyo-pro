#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        ss: [usize; n]
    };
    let mut max = 0;
    let mut min = std::usize::MAX;
    let mut sum = 0;
    for s in ss {
        max = max.max(s);
        min = min.min(s);
        sum += s;
    }
    // B = P(s_max - s_min)
    // A = (P*sum + n*Q) / n
    if max == min {
        println!("-1");
        return;
    }
    let p = b as f64 / (max - min) as f64;
    let q = a as f64 - p * (sum as f64) / (n as f64);
    println!("{} {}", p, q);
}

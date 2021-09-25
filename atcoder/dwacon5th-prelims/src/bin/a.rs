#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let avg = aa.iter().sum::<usize>() as f64 / n as f64;
    let mut min = std::f64::MAX;
    let mut rs = 0;
    for (i, a) in aa.into_iter().enumerate() {
        let diff = (a as f64 - avg).abs();
        if diff < min {
            min = diff;
            rs = i;
        }
    }
    println!("{}", rs);
}

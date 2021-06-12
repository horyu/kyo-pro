#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut xxxxx: [[usize; 5]; n]
    };
    let rs = xxxxx
        .iter_mut()
        .map(|v| (v.pop().unwrap() * 110 + v.iter().sum::<usize>() * 900) as f64 / 900.0)
        .fold(0.0, |m, v| v.max(m));
    println!("{}", rs);
}

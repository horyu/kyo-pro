#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [isize; n]
    };
    let rs = (-100..=100)
        .map(|i| aa.iter().map(|a| (i - a).pow(2)).sum::<isize>())
        .min()
        .unwrap();
    println!("{}", rs);
}

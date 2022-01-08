#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let rr = xxyy
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).pow(2u32) + (a.1 - b.1).pow(2u32))
        .max()
        .unwrap();
    let rs = (rr as f64).sqrt();
    println!("{}", rs);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let rr = xxyy
        .iter()
        .tuple_combinations()
        .map(|((x0, y0), (x1, y1))| (x0 - x1).pow(2) + (y0 - y1).pow(2))
        .max()
        .unwrap();
    println!("{}", (rr as f64).sqrt());
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        d: isize,
        xxyy: [(isize, isize); n]
    };
    let dd = d.pow(2);
    let rs = xxyy
        .into_iter()
        .filter(|(x, y)| x.pow(2) + y.pow(2) <= dd)
        .count();
    println!("{}", rs);
}

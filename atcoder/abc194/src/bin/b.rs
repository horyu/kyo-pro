#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        abab: [(usize, usize); n]
    };
    let mut min = std::usize::MAX;
    for (a, b) in &abab {
        min = min.min(a + b);
    }
    for arr in abab.iter().combinations(2) {
        let &(a1, b1) = arr[0];
        let &(a2, b2) = arr[1];
        min = min.min(a1.max(b2));
        min = min.min(a2.max(b1));
    }
    println!("{}", min);
}

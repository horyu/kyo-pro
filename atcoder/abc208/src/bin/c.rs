#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut iiaabb = aa
        .into_iter()
        .enumerate()
        .map(|ia| (ia.0, ia.1, 0))
        .collect_vec();
    iiaabb.sort_unstable_by_key(|&ia| ia.1);
    for iab in iiaabb[0..(k % n)].iter_mut() {
        iab.2 = 1;
    }
    iiaabb.sort_unstable_by_key(|&ia| ia.0);
    let min = k / n;
    for (_i, _a, b) in iiaabb {
        println!("{}", min + b);
    }
}

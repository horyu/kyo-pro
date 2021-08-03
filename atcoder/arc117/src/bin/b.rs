#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.push(0);
    const MOD: usize = 1_000_000_007;
    aa.sort_unstable();
    let rs = aa
        .into_iter()
        .tuple_windows()
        .fold(1, |acc, (a1, a2)| acc * (a2 - a1 + 1) % MOD);
    println!("{}", rs);
}

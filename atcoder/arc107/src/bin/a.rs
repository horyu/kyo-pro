#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        abc: [u128; 3]
    };
    const MOD: u128 = 998244353;
    let rs = abc
        .into_iter()
        .fold(1, |acc, n| (acc * n * (n + 1) / 2) % MOD);
    println!("{}", rs);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

const MOD: u128 = 1e9 as u128 + 7;

fn main() {
    input! {
        n: usize,
        k: u128,
        aa: [u128; n],
    };
    let mut ll = vec![0u128; n];
    let mut rr = vec![0u128; n];
    for (i, ai) in aa.iter().enumerate() {
        for aj in &aa[..i] {
            if aj > ai {
                ll[i] += 1;
            }
        }
        for aj in &aa[(i + 1)..] {
            if aj > ai {
                rr[i] += 1;
            }
        }
    }
    let mut rs = 0;
    for (l, r) in std::iter::zip(ll, rr) {
        // l, lrl, lrlrl, ...
        rs += ((2 * l + (r + l) * (k - 1)) * k / 2) % MOD;
        rs %= MOD;
    }
    println!("{rs}");
}

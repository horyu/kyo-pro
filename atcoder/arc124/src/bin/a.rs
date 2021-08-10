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
        cckk: [(char, Usize1); k]
    };
    const MOD: usize = 998244353;
    let mut a = vec![0; n];
    let mut b = vec![true; n];
    for (c, k) in cckk {
        for j in 0..n {
            b[k] = false;
            match c {
                'L' if k <= j => a[j] += 1,
                'R' if k >= j => a[j] += 1,
                _ => (),
            }
        }
    }
    let mut rs = 1;
    for (a, b) in a.into_iter().zip(b.into_iter()) {
        if b {
            rs = rs * a % MOD;
        }
    }
    println!("{}", rs);
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
    };
    const MOD: usize = 998244353;

    let h1 = h - 1;
    let h2 = h - 2;
    let w1 = w - 1;
    let w2 = w - 2;

    let mut a = 1;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    for _ in 0..k {
        let na = b * w1 + c * h1;
        let nb = a + b * w2 + d * h1;
        let nc = a + c * h2 + d * w1;
        let nd = b + c + d * (h2 + w2);
        a = na % MOD;
        b = nb % MOD;
        c = nc % MOD;
        d = nd % MOD;
    }
    let rs = match (x1 == x2, y1 == y2) {
        (true, true) => a,
        (true, false) => b,
        (false, true) => c,
        (false, false) => d,
    };
    println!("{}", rs);
}

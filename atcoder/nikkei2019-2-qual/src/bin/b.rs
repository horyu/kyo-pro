#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        dd: [usize; n]
    };
    let mut cc = vec![0usize; n];
    let mut max = std::usize::MIN;
    for &d in &dd {
        max = max.max(d);
        cc[d] += 1;
    }
    let mut rs = if dd[0] == 0 && cc[0] == 1 { 1 } else { 0 };
    for i in 0..max {
        rs = rs * mod_pow(cc[i], cc[i + 1], 998244353) % 998244353;
    }
    println!("{rs}");
}

// https://blog.spiralray.net/cp/modulo#i-8
fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;
    while n != 0 {
        if n.is_odd() {
            ans = ans * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ans
}

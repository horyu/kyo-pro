#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_roundings)]
#![feature(map_first_last)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 998244353;
fn main() {
    input! {
        x: usize
    };
    let mut rs = 1;
    let mut btm = BTreeMap::new();
    btm.insert(x, 1usize);
    while let Some((k, c)) = btm.pop_last() {
        if k <= 4 {
            rs = rs * mod_pow(k, c, MOD) % MOD;
        } else {
            let p = k.div_ceil(2);
            let q = k.div_floor(2);
            *btm.entry(p).or_insert(0) += c;
            *btm.entry(q).or_insert(0) += c;
        }
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

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    const MOD: usize = 1e9 as usize + 7;
    let mut rs = (0..(n.min(2))).fold(1, |acc, i| acc * k.saturating_sub(i) % MOD);
    rs = rs * mod_pow(k.saturating_sub(2), n.saturating_sub(2), MOD) % MOD;
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

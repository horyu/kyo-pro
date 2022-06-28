#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_log)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut l: u128,
        r: u128,
    };
    const MOD: u128 = 1e9 as u128 + 7;
    let mut rs = 0u128;
    while l <= r {
        let keta = l.log10() + 1;
        let rr = (10u128.pow(keta) - 1).min(r);
        // 初項l 末項rr, 項数rr-(l-1)
        rs = (rs + (keta as u128) * (l + rr) * (rr - l + 1) / 2 % MOD) % MOD;

        l = 10u128.pow(keta);
    }
    println!("{rs}");
}

#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    const MOD: u128 = 998244353;
    let s = s
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as u128)
        .collect_vec();
    let n = s.len();
    if n == 1 {
        println!("{}", s[0]);
        return;
    }
    let mut pow2 = vec![1u128];
    let mut pow10 = vec![1u128];
    for i in 0..(n - 1) {
        pow2.push(pow2[i] * 2 % MOD);
        pow10.push(pow10[i] * 10 % MOD);
    }
    let mut sum = 0;
    for i in 0..n {
        sum = (sum + pow2[i] * pow10[n - 1 - i] * s[i]) % MOD;
    }
    let mut ss = 0;
    for i in 0..(n - 1) {
        ss += s[i];
        sum = (sum + pow2[i] * pow10[n - 2 - i] * ss) % MOD;
    }
    println!("{}", sum);
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: isize = 998244353;

fn main() {
    input! {
        n: Usize1
    };
    let mut pre = vec![1isize; 5];
    for _ in 0..n {
        let mut crr = pre.clone();
        for j in 0..=3 {
            crr[j] += pre[j + 1];
        }
        for j in 1..=3 {
            crr[j] += pre[j - 1];
        }
        crr[4] += 2 * pre[3];
        for j in 0..=4 {
            crr[j] %= MOD;
        }
        pre = crr;
    }
    let last = pre.pop().unwrap();
    let rs = (last + 2 * pre.into_iter().sum::<isize>()) % MOD;
    println!("{rs}");
}

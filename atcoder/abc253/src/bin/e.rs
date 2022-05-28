#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut dp = vec![1; m];
    for _ in 1..n {
        let mut new_dp = vec![0; m];
        let mut ss = vec![0];
        for i in 0..m {
            ss.push((ss[i] + dp[i]) % MOD);
        }
        for i in 0..m {
            if k == 0 {
                new_dp[i] = ss[m];
            } else {
                new_dp[i] =
                    (MOD + ss[m] - ss[(i + k).min(m)] + ss[(i + 1).saturating_sub(k)]) % MOD;
            }
        }
        dp = new_dp;
    }
    let rs = dp.into_iter().fold(0, |acc, x| (acc + x) % MOD);
    println!("{rs}");
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 998244353;
fn main() {
    // 貰うDP
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let ss = dp[i - 1].iter().fold(vec![0], |mut vv, d| {
            vv.push(vv.last().unwrap() + d);
            vv
        });
        for j in i..=k {
            dp[i][j] = (ss[j] - ss[j.saturating_sub(m)]) % MOD;
        }
    }
    let rs = dp[n].iter().fold(0, |acc, x| (acc + x) % MOD);
    println!("{rs}");
}

#[allow(dead_code)]
fn main1() {
    // 配るDP
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..k {
            for w in 1..=m {
                if j + w <= k {
                    dp[i + 1][j + w] += dp[i][j];
                    dp[i + 1][j + w] %= MOD;
                }
            }
        }
    }
    let rs = dp[n].iter().sum::<usize>() % MOD;
    println!("{rs}");
}

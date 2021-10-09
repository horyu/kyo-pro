#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    const MOD: usize = 998244353;
    let mut dp = vec![vec![0; 3001]; n];
    for k in aa[0]..=bb[0] {
        dp[0][k] = 1;
    }
    for (i, (&a, &b)) in aa[1..].iter().zip(bb[1..].iter()).enumerate() {
        if a > b {
            println!("0");
            return;
        }
        let next = i + 1;
        dp[next][a] = dp[i][0..=a].iter().sum::<usize>() % MOD;
        for k in (a + 1)..=b {
            dp[next][k] = (dp[next][k - 1] + dp[i][k]) % MOD;
        }
    }
    let rs = dp[n - 1].iter().sum::<usize>() % MOD;
    println!("{}", rs);
}

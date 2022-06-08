#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

const MOD: usize = 10usize.pow(9u32) + 7;
fn main() {
    input! {
        k: usize,
    };
    if k % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..=k {
        for j in 1..=i.min(9) {
            dp[i] += dp[i - j];
            dp[i] %= MOD;
        }
    }
    println!("{}", dp[k]);
}

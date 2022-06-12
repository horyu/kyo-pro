#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        l: usize,
    };
    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        dp[i] += dp[i - 1];
        if l <= i {
            dp[i] += dp[i - l];
        }
        dp[i] %= 1e9 as usize + 7;
    }
    println!("{}", dp[n]);
}

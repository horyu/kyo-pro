#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let mut rs = std::usize::MAX;
    for k in 0..=1 {
        aa.rotate_right(k);
        // [o, x]
        let mut dp = [aa[0], std::usize::MAX >> 1];
        for &a in &aa[1..] {
            let new_dp = [dp[0].min(dp[1]) + a, dp[0]];
            dp = new_dp;
        }
        rs = rs.min(*dp.iter().min().unwrap());
    }
    println!("{rs}");
}

#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    let mut dp = vec![0; n];
    dfs(&mut dp, &vvv, 0, std::usize::MAX);
    let mut rs = 0;
    for &d in &dp[1..] {
        rs += d * (n - d);
    }
    println!("{rs}");
}

fn dfs(dp: &mut Vec<usize>, vvv: &[Vec<usize>], pos: usize, pre: usize) {
    dp[pos] = 1;
    for &i in &vvv[pos] {
        if i != pre {
            dfs(dp, vvv, i, pos);
            dp[pos] += dp[i];
        }
    }
}

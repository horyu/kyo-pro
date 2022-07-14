#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        cc: [char; n],
        aabb: [(Usize1, Usize1); n - 1],
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }

    let mut dp = vec![vec![0; 3]; n];
    dfs(&vvv, &cc, &mut dp, 0, std::usize::MAX);

    println!("{}", dp[0][2]);
}

const MOD: usize = 1000000007;

fn dfs(vvv: &[Vec<usize>], cc: &[char], dp: &mut Vec<Vec<usize>>, pos: usize, pre: usize) {
    let mut x = 1;
    let mut y = 1;
    for &i in &vvv[pos] {
        if i == pre {
            continue;
        }
        dfs(vvv, cc, dp, i, pos);
        if cc[pos] == 'a' {
            x *= dp[i][0] + dp[i][2];
            y *= dp[i][0] + dp[i][1] + 2 * dp[i][2];
        } else {
            x *= dp[i][1] + dp[i][2];
            y *= dp[i][0] + dp[i][1] + 2 * dp[i][2];
        }
        x %= MOD;
        y %= MOD;
    }
    if cc[pos] == 'a' {
        dp[pos][0] = x;
        dp[pos][2] = (MOD + y - x) % MOD;
    } else {
        dp[pos][1] = x;
        dp[pos][2] = (MOD + y - x) % MOD;
    }
}

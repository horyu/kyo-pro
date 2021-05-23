#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut k: usize,
    };
    let mut dp = [[0usize; 31]; 31];
    for i in 0usize..31 {
        dp[0][i] = 1;
        dp[i][0] = 1;
    }
    for i in 1..=a {
        for j in 1..=b {
            dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
        }
    }
    let mut s = String::new();
    while (a > 0) && (b > 0) {
        if k <= dp[a - 1][b] {
            s.push('a');
            a -= 1;
        } else {
            k -= dp[a - 1][b];
            s.push('b');
            b -= 1;
        }
    }
    s.push_str(&"a".repeat(a));
    s.push_str(&"b".repeat(b));
    println!("{}", s);
}

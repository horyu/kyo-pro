#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        cc: [usize; k],
    };
    const MOD: usize = 1e9 as usize + 7;
    const LOG: usize = 61;
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/005-03.cpp

    let mut ten = [0; LOG];
    for i in 0..LOG {
        ten[i] = mod_pow(10, 1 << i, b);
    }

    let mut dp = vec![vec![0; 1009]; LOG + 1];
    let mut ans = dp.clone();

    for c in cc.iter().copied() {
        dp[0][c % b] += 1;
    }

    for i in 0..LOG {
        for j in 0..b {
            for k in 0..b {
                let next = (j * ten[i] + k) % b;
                dp[i + 1][next] += dp[i][j] * dp[i][k];
                dp[i + 1][next] %= MOD;
            }
        }
    }

    ans[0][0] = 1;
    for i in 0..LOG {
        if 0 != n & (1 << i) {
            for j in 0..b {
                for k in 0..b {
                    let next = (j * ten[i] + k) % b;
                    ans[i + 1][next] += ans[i][j] * dp[i][k];
                    ans[i + 1][next] %= MOD;
                }
            }
        } else {
            for j in 0..b {
                ans[i + 1][j] = ans[i][j];
            }
        }
    }

    let rs = ans[LOG][0];
    println!("{rs}");
}

fn mod_pow(mut x: usize, mut n: usize, m: usize) -> usize {
    x %= m;
    let mut ans = 1;
    while n != 0 {
        if n.is_odd() {
            ans = ans * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ans
}

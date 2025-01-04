#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        aabb: [(Usize1, usize); n],
    };
    // https://atcoder.jp/contests/abc375/editorial/11140
    let sum = aabb.iter().map(|ab| ab.1).sum::<usize>();
    if sum % 3 != 0 {
        println!("-1");
        return;
    }
    let target = sum / 3;
    dbg!(sum, target);
    const A: usize = 501;
    let mut dp = vec![vec![1usize << 60; A]; A];
    dp[0][0] = 0;
    for  (a, b) in aabb.iter().copied() {
        let mut new_dp = vec![vec![1usize << 60; A]; A];
        for i in 0..A {
            for j in 0..A {
                match a {
                    0 => {
                        if i + b < A {
                            new_dp[i + b][j] = new_dp[i + b][j].min(dp[i][j]);
                        }
                        if j + b < A {
                            new_dp[i][j + b] = new_dp[i][j + b].min(dp[i][j] + 1);
                        }
                        new_dp[i][j] = new_dp[i][j].min(dp[i][j] + 1);
                    }
                    1 => {
                        if i + b < A {
                            new_dp[i + b][j] = new_dp[i + b][j].min(dp[i][j] + 1);
                        }
                        if j + b < A {
                            new_dp[i][j + b] = new_dp[i][j + b].min(dp[i][j]);
                        }
                        new_dp[i][j] = new_dp[i][j].min(dp[i][j] + 1);
                    }
                    _ => {
                        if i + b < A {
                            new_dp[i + b][j] = new_dp[i + b][j].min(dp[i][j] + 1);
                        }
                        if j + b < A {
                            new_dp[i][j + b] = new_dp[i][j + b].min(dp[i][j] + 1);
                        }
                        new_dp[i][j] = new_dp[i][j].min(dp[i][j]);
                    }
                }
            }
        }
        dp = new_dp;
    }
    let rs = dp[target][target];
    if rs == 1usize << 60 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}

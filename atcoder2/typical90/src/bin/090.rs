#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![allow(dead_code)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: usize = 998244353;
fn main() {
    input! {n: usize, k: usize};
    if k == 1 {
        main12(n, k);
    } else {
        // main3(n, k);
        main4(n, k);
    }
}

fn main4(n: usize, k: usize) {
    assert!(n <= 100 && k <= 100);
    // do[h][l][r]: 条件を満たす部分列 al,..,ar の値が全てh以上であるものの個数
    let mut dp = vec![vec![vec![ModInt998244353::default(); n + 1]; n]; k + 1];
    for h in 0..=k {
        for i in 0..n {
            dp[h][i][i] += 1;
        }
    }
    for h in (0..=k).rev() {
        for l in 0..n {
            for r in (l + 1)..=n {
                if k < h * (r - l) {
                    continue;
                }
                dp[h][l][r] = dp[h][l][r - 1];
                if h != k {
                    for i in l..r {
                        dp[h][l][r] = dp[h][l][r]
                            + dp[h + 1][i][r] * dp[h][l][if i != l { i - 1 } else { l }];
                    }
                }
            }
        }
    }
    let rs = dp[0][0][n];
    println!("{rs}");
}

fn main3(n: usize, k: usize) {
    assert!(n <= 6 && k <= 6);
    let rs = (0..n)
        .map(|_| 0..=k)
        .multi_cartesian_product()
        .filter(|aa| {
            for (l, al) in aa.iter().enumerate() {
                let mut min = al;
                for (r, ar) in aa.iter().enumerate().skip(l + 1) {
                    min = min.min(ar);
                    if k < min * (r - l + 1) {
                        return false;
                    }
                }
            }
            true
        })
        .count();
    println!("{rs}");
}

fn main12(n: usize, k: usize) {
    assert!(k <= 1);
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/090-02.cpp
    let mut rs = ndarray::arr2(&[[1usize, 0], [0, 1]]);
    let mut mat = ndarray::arr2(&[[1usize, 1], [1, 0]]);
    let mut b = n;
    while 0 < b {
        if b & 1 == 1 {
            rs = rs.dot(&mat);
            rs %= MOD;
        }
        mat = mat.dot(&mat);
        mat %= MOD;
        b >>= 1;
    }
    let rs = (rs[[0, 0]] + rs[[1, 0]]) % MOD;
    println!("{rs}");
}

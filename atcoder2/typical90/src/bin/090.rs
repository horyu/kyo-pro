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
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: usize = 998244353;
fn main() {
    input! {n: usize, k: usize};
    // if k == 1 {
    //     main12(n, k);
    // } else {
    //     // main3(n, k);
    //     // main4(n, k);
    //     // main5(n, k);
    // }
    main6(n, k);
}

fn main6(n: usize, k: usize) {
    assert!(n <= 1e5 as usize);
    let mut dp = vec![vec![]; k + 1];
    dp[k] = vec![ModInt998244353::new(1); 3];
    for i in (0..k).rev() {
        let limit = if i == 0 { n } else { n.min(k / i) };
        let mut cc = vec![ModInt998244353::default(); dp[i + 1].len()];
        cc[0] += 1;
        for j in 1..dp[i + 1].len() {
            cc[j] = -dp[i + 1][j];
        }
        dp[i] = polynomial_inverse(&cc, limit + 2);
    }
    let rs = dp[0][n + 1];
    println!("{rs}");
}

fn polynomial_inverse(cc: &[ModInt998244353], l: usize) -> Vec<ModInt998244353> {
    let n = cc.len();
    let mut aa = vec![ModInt998244353::default(); 2];
    aa[0] += 1;
    let mut level = 0;
    while (1 << level) < l {
        let cs = (2 << level).min(n);
        let pp = ac_library::convolution(&aa, &cc[..cs]);
        let mut qq = vec![ModInt998244353::default(); 2 << level];
        qq[0] += 1;
        for j in (1 << level)..(2 << level) {
            qq[j] = -pp[j];
        }
        aa = ac_library::convolution(&aa, &qq);
        aa.resize(4 << level, Default::default());
        level += 1;
    }
    aa.resize(l, Default::default());
    aa
}

fn main5(n: usize, k: usize) {
    assert!(n <= 10000 && k <= 10000);
    // do[h][m]: 条件を満たす長さmの数列で値が全てh以上であるものの個数
    let mut dp = vec![vec![]; k + 2];
    dp[k] = vec![0, 1];
    for i in (0..k).rev() {
        let limit = if i == 0 { n } else { n.min(k / i) };
        dp[i].resize(limit + 1, 0);
        for j in 1..=limit {
            dp[i][j] = if j != 1 { dp[i][j - 1] } else { 1 };
            for k in 2.. {
                if !(k <= j + 1 && k <= dp[i + 1].len()) {
                    break;
                }
                dp[i][j] =
                    (dp[i][j] + dp[i + 1][k - 1] * if k < j { dp[i][j - k] } else { 1 }) % MOD;
            }
        }
    }
    let rs = dp[0][n];
    println!("{rs}");
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

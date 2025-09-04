#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        aa: [usize; n],
    };
    let mut rs = ModInt998244353::default();
    for k in 1..=n {
        let mut dp = vec![vec![ModInt998244353::default(); k]; k + 1];
        dp[0][0] = 1.into();
        for a in aa.iter().copied() {
            for i in (0..k).rev() {
                for j in 0..k {
                    let tmp = dp[i][j];
                    dp[i + 1][(j + a) % k] += tmp;
                }
            }
        }
        rs += dp[k][0];
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main3() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    let mut rs = ModInt998244353::default();
    for size in 1..=n {
        let mut hm = HashMap::<_, _>::from_iter([((0, 0), ModInt998244353::new(1))]);
        for a in aa.iter().copied() {
            let mut new_hm = hm.clone();
            for ((m, s), v) in hm {
                if s + 1 == size {
                    if (m + a) % size == 0 {
                        rs += v;
                    }
                } else {
                    *new_hm.entry(((m + a) % size, s + 1)).or_default() += v;
                }
            }
            hm = new_hm;
        }
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        aa: [usize; n],
    };
    // https://blog.hamayanhamayan.com/entry/2016/08/29/141910
    let mut rs = ModInt998244353::new(n);
    for num in 2..=n {
        // dp[i][j][k] = i個目までにj個選んで総和の余りがkである組合せ
        let mut dp = vec![vec![vec![ModInt998244353::new(0); num]; num + 2]; n + 1];
        dp[0][0][0] += 1;
        for i in 0..n {
            for j in 0..=num {
                for k in 0..num {
                    let x = dp[i][j][k];
                    dp[i + 1][j][k] += x;
                    dp[i + 1][j + 1][(k + aa[i]) % num] += x;
                }
            }
        }
        rs += dp[n][num][0];
    }
    println!("{rs}");
}

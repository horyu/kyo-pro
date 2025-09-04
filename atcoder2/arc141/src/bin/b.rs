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
        m: usize,
    };
    // https://atcoder.jp/contests/arc141/submissions/32087646
    if 60 < n {
        println!("0");
        return;
    }
    let l = m.ilog2() as usize + 1;
    let mut dp = vec![vec![ModInt998244353::default(); 61]; 61];
    dp[0][0] = 1.into();
    for i in 0..60 {
        for j in 1..=l {
            let mul = if j == l {
                m + 1 - (1 << (j - 1))
            } else {
                1 << (j - 1)
            };
            for k in 0..j {
                dp[i + 1][j] = dp[i + 1][j] + dp[i][k];
            }
            dp[i + 1][j] *= mul;
        }
    }
    let mut rs = ModInt998244353::default();
    for j in 1..=l {
        rs += dp[n][j];
    }
    println!("{rs}");
}

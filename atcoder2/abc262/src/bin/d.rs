#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use ac_library_rs::ModInt998244353;
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
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

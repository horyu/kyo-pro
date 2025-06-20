#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
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
        t: usize,
        nnkk: [(usize, usize); t],
    };
    // https://atcoder.jp/contests/abc406/editorial/13071
    for (n, k) in nnkk {
        let l = n.ilog2() as usize + 1; // n のビット長
        // i ビット目までを埋めた時点で N より小さいことが確定している非負整数であって，
        // 立っているビットが上から i ビット目までであり，
        // dp1​(i,j)：立っているビット数が j であるものの個数
        // dp2​(i,j)：立っているビット数が j であるものの総和
        let mut dp1 = vec![vec![ModInt998244353::default(); k + 1]; l];
        let mut dp2 = dp1.clone();
        dp1[0][0] += 1;

        // N未満が確定している場合の遷移
        for i in 0..l {
            for j in 0..=k {
                dp1[i + 1][j] = dp1[i + 1][j] + dp1[i][j];
                dp2[i + 1][j] = dp2[i + 1][j] + dp2[i][j];
                if j + 1 <= k {
                    dp1[i + 1][j + 1] = dp1[i + 1][j + 1] + dp1[i][j];
                    dp2[i + 1][j + 1] = dp2[i + 1][j + 1] + dp1[i][j] * (2usize << (k - 1 - i));
                }
            }
        }
        // TODO
    }
    // println!("{rs}");
}

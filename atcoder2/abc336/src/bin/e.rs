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
        nn: Bytes,
    };
    // https://atcoder.jp/contests/abc336/editorial/9055
    let nn = nn.into_iter().map(|n| (n - b'0') as usize).collect_vec();
    let len = nn.len();
    let mut rs = 0usize;
    for s in 1..=(9 * 14) {
        let mut dp = vec![vec![vec![[0usize; 2]; s]; s + 1]; len + 1];
        dp[0][0][0][1] = 1;
        for d in 0..len {
            for i in 0..=s {
                for j in 0..s {
                    for f in 0..2 {
                        for t in 0..10 {
                            if s < i + t {
                                break;
                            }
                            if (f == 1) && (nn[d] < t) {
                                break;
                            }
                            dp[d + 1][i + t][(j * 10 + t) % s][f & usize::from(nn[d] == t)] +=
                                dp[d][i][j][f];
                        }
                    }
                }
            }
        }
        rs += dp[len][s][0][0] + dp[len][s][0][1];
    }
    println!("{rs}");
}

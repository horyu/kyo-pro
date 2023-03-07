#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: Bytes,
    };
    // 31 = 3*10 + 1 | 4*10 - 9
    // 36 = 3*10 + 6 | 4*10 - 4
    // 39 = 3*10 + 9 | 4*10 - 1
    // 81 = 8*10 + 1 | 9*10 - 9 | 1*100 - 1*10 - 9
    // 86 = 8*10 + 6 | 9*10 - 4 | 1*100 - 1*10 - 4
    // 89 = 8*10 + 9 | 9*10 - 1 | 1*100 - 1*10 - 1

    // https://blog.hamayanhamayan.com/entry/2020/02/17/001028
    let aa = n
        .iter()
        .copied()
        .map(|n| (n - b'0') as usize)
        .rev()
        .collect_vec();
    let len = aa.len();
    let mut dp = vec![[!0usize; 2]; len + 1];
    dp[0][0] = 0;
    for (i, a) in aa.into_iter().enumerate() {
        for d in 0..2 {
            if dp[i][d] == !0 {
                continue;
            }
            let c = a + d;
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][d] + c);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][d] + 10 - c);
        }
    }
    let rs = dp[len][0].min(dp[len][1] + 1);
    println!("{rs}");
}

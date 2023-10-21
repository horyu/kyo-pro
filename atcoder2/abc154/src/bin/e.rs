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
        n: Bytes,
        k: usize,
    };
    let len = n.len();
    let nn = n.into_iter().map(|n| (n - b'0') as usize).collect_vec();
    // https://blog.hamayanhamayan.com/entry/2020/02/09/225141
    let mut dp = vec![vec![vec![0; k + 2]; 2]; len + 1];
    dp[0][0][0] = 1;
    for dgt in 0..len {
        for is_less in 0..2 {
            for kk in 0..=k {
                let n = nn[dgt];
                for nxt in 0..10 {
                    if n < nxt && is_less == 0 {
                        continue;
                    }
                    let is_less2 = is_less | (nxt < n) as usize;
                    let k2 = kk + (0 < nxt) as usize;

                    dp[dgt + 1][is_less2][k2] += dp[dgt][is_less][kk];
                }
            }
        }
    }

    let rs = dp[len][0][k] + dp[len][1][k];
    println!("{rs}");
}

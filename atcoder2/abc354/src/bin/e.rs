#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        aabb: [(usize, usize); n],
    };
    // https://atcoder.jp/contests/abc354/editorial/10034
    // BitDP
    let mut dp = vec![false; 1 << n];
    for i in 1..(1 << n) {
        let mut tf = false;
        for j in 0..n {
            for k in (j + 1)..n {
                if ((i >> j) & 1) == 1 && (i >> k) & 1 == 1 {
                    let (a, b) = aabb[j];
                    let (c, d) = aabb[k];
                    if (a == c || b == d) && !dp[i ^ (1 << j) ^ (1 << k)] {
                        tf = true;
                    }
                }
            }
        }
        dp[i] = tf;
    }
    let rs = ["Aoki", "Takahashi"][dp[(1 << n) - 1] as usize];
    println!("{rs}");
}

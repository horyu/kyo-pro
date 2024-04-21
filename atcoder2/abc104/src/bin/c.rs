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
        d: usize,
        g: usize,
        ppcc: [(usize, usize); d],
    };
    const N: usize = 201010;

    // https://blog.hamayanhamayan.com/entry/2018/08/05/232112
    let g = g / 100;
    let pp = ppcc.iter().copied().map(|(p, _)| p).collect_vec();
    let cc = ppcc.iter().copied().map(|(_, c)| c / 100).collect_vec();

    let mut dp = vec![vec![1 << 60; N * 2]; d + 1];
    dp[0][0] = 0;

    for i in 0..d {
        for point in 0..N {
            for j in 0..pp[i] {
                dp[i + 1][point + (i + 1) * j] =
                    dp[i + 1][point + (i + 1) * j].min(dp[i][point] + j);
            }
            dp[i + 1][point + (i + 1) * pp[i] + cc[i]] =
                dp[i + 1][point + (i + 1) * pp[i] + cc[i]].min(dp[i][point] + pp[i]);
        }
    }

    let mut rs = !0;
    for point in g..N {
        rs = rs.min(dp[d][point]);
    }
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        d: usize,
        g: usize,
        ppcc: [(usize, usize); d],
    };
    let mut rs = usize::MAX;
    for ttff in (0..d).map(|_| [false, true]).multi_cartesian_product() {
        let mut cnt = 0;
        let mut score = 0;
        for (i, tf) in ttff.iter().copied().enumerate() {
            if tf {
                let (p, c) = ppcc[i];
                cnt += p;
                score += p * 100 * (i + 1) + c;
            }
        }
        if g <= score {
            rs = rs.min(cnt);
            continue;
        }
        for (i, tf) in ttff.iter().copied().enumerate().rev() {
            if !tf {
                let (p, c) = ppcc[i];
                let diff = g - score;
                let pp = diff.div_ceil(100 * (i + 1));
                if pp < p {
                    cnt += pp;
                    score += pp * 100 * (i + 1);
                }
                break;
            }
        }
        if g <= score {
            rs = rs.min(cnt);
        }
    }
    println!("{rs}");
}

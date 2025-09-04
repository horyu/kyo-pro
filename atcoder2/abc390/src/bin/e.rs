#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        x: usize,
        vvaacc: [(Usize1, isize, usize); n],
    };
    // https://atcoder.jp/contests/abc390/editorial/12052
    const M: usize = 5000;
    let mut dp = vec![vec![-2e9 as isize; M + 1]; 3];
    for i in 0..3 {
        dp[i][0] = 0;
    }
    for (v, a, c) in vvaacc {
        for j in (c..=x).rev() {
            dp[v][j] = dp[v][j].max(dp[v][j - c] + a);
        }
    }
    for i in 0..3 {
        for j in 0..x {
            dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
        }
    }
    let mut idx = [0; 3];
    for i in 0..x {
        if dp[0][idx[0]] <= dp[1][idx[1]] && dp[0][idx[0]] <= dp[2][idx[2]] {
            idx[0] += 1;
        } else if dp[1][idx[1]] <= dp[0][idx[0]] && dp[1][idx[1]] <= dp[2][idx[2]] {
            idx[1] += 1;
        } else {
            idx[2] += 1;
        }
    }
    let rs = izip!(dp, idx).map(|(dp, idx)| dp[idx]).min().unwrap();
    println!("{rs}");
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        x: usize,
        vvaacc: [(Usize1, usize, usize); n],
    };
    let mut aaa = vec![vec![0; x + 1]; 3];
    for (v, a, c) in vvaacc {
        let aa = &mut aaa[v];
        for i in ((c + 1)..=x).rev() {
            if 0 < aa[i - c] {
                aa[i] = aa[i].max(aa[i - c] + a);
            }
        }
        aa[c] = aa[c].max(a);
    }
    let bbttmm = aaa
        .into_iter()
        .map(|aa| {
            let mut btm = BTreeMap::new();
            let mut max = 0;
            for (c, a) in aa.into_iter().enumerate() {
                if max < a {
                    btm.insert(a, c);
                }
                max = max.max(a);
            }
            btm
        })
        .collect_vec();
    // for btm in &bbttmm {
    //     eprintln!("{btm:?}");
    // }
    let mut ok = 0;
    let mut ng = bbttmm
        .iter()
        .flat_map(|btm| btm.keys().max())
        .min()
        .unwrap()
        + 1;
    while 1 < ng - ok {
        let mid = ok.midpoint(ng);
        let mut sum = 0;
        for btm in bbttmm.iter() {
            if let Some((_a, c)) = btm.range(mid..).next() {
                sum += c;
            } else {
                sum = x + 1;
                break;
            }
        }
        if sum <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let rs = ok;
    println!("{rs}");
}

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

// ダイクストラ
fn main() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![]; n];
    for (i, (a, b, c)) in aabbcc.iter().copied().enumerate() {
        g[a].push((i, b, c));
        g[b].push((i, a, c));
    }
    let mut ttff = vec![false; m];
    for start in 0..n {
        let mut bh = BinaryHeap::new();
        bh.push((R(0), !0, start));
        let mut cc = vec![!0; n];
        cc[start] = 0;
        while let Some((R(c), i, from)) = bh.pop() {
            if cc[from] < c {
                continue;
            }
            if i <= m {
                ttff[i] = true;
            }
            for &(j, next, next_c) in &g[from] {
                if next == from {
                    continue;
                }
                if c + next_c <= cc[next] {
                    cc[next] = c + next_c;
                    bh.push((R(c + next_c), j, next));
                }
            }
        }
    }
    let rs = ttff.iter().filter(|&&b| !b).count();
    println!("{rs}");
}

// ワーシャルフロイド
#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![!0usize >> 3; n]; n];
    for &(a, b, c) in &aabbcc {
        g[a][b] = c;
        g[b][a] = c;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    let rs = aabbcc.into_iter().filter(|&(a, b, c)| g[a][b] != c).count();
    println!("{rs}");
}

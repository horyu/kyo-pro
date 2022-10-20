#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aabbtt: [(Usize1, Usize1, usize); m],
    };
    let mut g = vec![vec![!0usize >> 2; n]; n];
    for (a, b, t) in aabbtt {
        g[a][b] = t;
        g[b][a] = t;
    }
    for i in 0..n {
        g[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }
    let rs = g
        .into_iter()
        .map(|vv| vv.into_iter().max().unwrap())
        .min()
        .unwrap();
    println!("{rs}");
}

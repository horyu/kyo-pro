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
        aabbcc: [(Usize1, Usize1, usize); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (u, v, c) in aabbcc.iter().copied() {
        g[u].push((v, c));
        g[v].push((u, c));
    }
    // 木の全頂点を通る最短経路長 = オイラーツアーの長さ - 木の直径
    // let rs = dfs(&g, 0, !0) - diameter(&g);
    let rs = 2 * aabbcc.into_iter().fold(0, |acc, abc| acc + abc.2) - diameter(&g);
    println!("{rs}");
}

// // iを頂点とする部分木全てを通って戻って来る最短経路長
// fn dfs(g: &[Vec<(usize, usize)>], i: usize, p: usize) -> usize {
//     let mut rs = 0;
//     for &(j, c) in &g[i] {
//         if j == p {
//             continue;
//         }
//         rs += 2 * c + dfs(g, j, i);
//     }
//     rs
// }

// 木の直径
fn diameter(g: &[Vec<(usize, usize)>]) -> usize {
    let mut start = 0;
    let mut dd = vec![!0; g.len()];
    for i in 0..2 {
        if i == 1 {
            dd = vec![!0; g.len()];
        }
        let mut qq = VecDeque::new();
        qq.push_back(start);
        dd[start] = 0;
        while let Some(u) = qq.pop_front() {
            for &(v, c) in &g[u] {
                if dd[v] == !0 {
                    dd[v] = dd[u] + c;
                    qq.push_back(v);
                }
            }
        }
        start = dd.iter().position_max().unwrap();
    }
    dd[start]
}

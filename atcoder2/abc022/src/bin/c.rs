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
        n: usize,
        m: usize,
        uuvvll: [(Usize1, Usize1, usize); m],
    };
    let mut rs = usize::MAX;
    let mut g = vec![vec![]; n];
    let mut ww = vec![];
    for (u, v, l) in uuvvll.iter().copied() {
        if u == 0 {
            ww.push((v, l));
        } else {
            g[u].push((v, l));
            g[v].push((u, l));
        }
    }
    for (i, (s, sl)) in ww.iter().copied().enumerate() {
        let hm = pathfinding::prelude::dijkstra_all(&s, |&f| g[f].clone());
        for (t, tl) in ww[(i + 1)..].iter().copied() {
            if let Some((_, l)) = hm.get(&t).copied() {
                rs = rs.min(sl + l + tl);
            }
        }
    }
    // for (i, i_l) in g[0].iter().copied() {
    //     // https://theory-and-me.hatenablog.com/entry/2019/09/08/182442
    //     let mut dist = vec![usize::MAX; n];
    //     let mut bh = BinaryHeap::new();
    //     dist[i] = 0;
    //     bh.push((std::cmp::Reverse(0), i));
    //     while let Some((std::cmp::Reverse(p_l), p)) = bh.pop() {
    //         if dist[p] < p_l {
    //             continue;
    //         }
    //         for (q, q_l) in g[p].iter().copied() {
    //             if p == i && q == 0 {
    //                 continue;
    //             }
    //             let q_ll = p_l + q_l;
    //             if q_ll < dist[q] {
    //                 dist[q] = q_ll;
    //                 bh.push((std::cmp::Reverse(q_ll), q));
    //             }
    //         }
    //     }
    //     for (j, j_l) in g[0].iter().copied() {
    //         if i < j {
    //             rs = rs.min(dist[j].saturating_add(i_l + j_l));
    //         }
    //     }
    // }
    if rs == usize::MAX {
        println!("-1");
    } else {
        println!("{rs}");
    }
}

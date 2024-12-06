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
        n: usize,
        m: usize,
        uuvvtt: [(Usize1, Usize1, usize); m],
        q: usize,
    };
    // ワーシャルフロイド法
    let mut dist = vec![vec![1usize << 60; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for (u, v, t) in uuvvtt.iter().copied() {
        let d = dist[u][v].min(t);
        dist[u][v] = d;
        dist[v][u] = d;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! { k: usize, bb: [Usize1; k] };
        let mut rs = !0;
        // 橋の利用順で全探索
        for bb in bb.iter().copied().permutations(k) {
            // 橋の向きで全探索
            for bits in 0..(1 << k) {
                let mut tmp = 0;
                let mut last = 0;
                for i in 0..k {
                    let (mut u, mut v, t) = uuvvtt[bb[i]];
                    if bits & (1 << i) != 0 {
                        (u, v) = (v, u);
                    }
                    tmp += dist[last][u] + t;
                    last = v;
                }
                tmp += dist[last][n - 1];
                rs = rs.min(tmp);
            }
        }
        println!("{rs}");
    }
}

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
        q: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut qq = vec![];
    let mut edges = vec![true; m];
    for _ in 0..q {
        input! {q: Usize1};
        if q == 0 {
            input! { idx: Usize1 };
            qq.push((q, idx, 0));
            edges[idx] = false;
        } else {
            input! { x: Usize1, y: Usize1 };
            qq.push((q, x, y));
        }
    }
    let mut ddd = vec![vec![1usize << 60; n]; n];
    for i in 0..n {
        ddd[i][i] = 0;
    }
    for (tf, &(a, b, c)) in izip!(edges, &aabbcc) {
        if tf {
            ddd[a][b] = c;
            ddd[b][a] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in (i + 1)..n {
                ddd[i][j] = ddd[i][j].min(ddd[i][k] + ddd[k][j]);
                ddd[j][i] = ddd[i][j];
            }
        }
    }
    let mut rrss = vec![];
    for (q, x, y) in qq.into_iter().rev() {
        if q == 0 {
            let (a, b, c) = aabbcc[x];
            if c < ddd[a][b] {
                ddd[a][b] = c;
                ddd[b][a] = c;
                for i in 0..n {
                    for j in (i + 1)..n {
                        ddd[i][j] = ddd[i][j]
                            .min(ddd[i][a] + c + ddd[b][j])
                            .min(ddd[i][b] + c + ddd[a][j]);
                        ddd[j][i] = ddd[i][j];
                    }
                }
            }
            continue;
        }
        let d = ddd[x][y];
        if 1usize << 60 <= d {
            rrss.push(-1);
        } else {
            rrss.push(d as i64);
        }
    }
    let rs = rrss.into_iter().rev().join("\n");
    println!("{rs}");
}

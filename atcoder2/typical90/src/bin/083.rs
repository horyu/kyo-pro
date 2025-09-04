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
        m: usize,
        aabb: [(Usize1, Usize1); m],
        q: usize,
        xxyy: [(Usize1, usize); q],
    };
    let xx = xxyy.iter().copied().map(|xy| xy.0).collect_vec();
    let yy = xxyy.iter().copied().map(|xy| xy.1).collect_vec();

    let mut g = (0..n).map(|i| HashSet::from([i])).collect_vec();
    for (a, b) in aabb.iter().copied() {
        g[a].insert(b);
        g[b].insert(a);
    }

    let mut cc = vec![1; n];
    let b = q.sqrt();
    for i in 0..b {
        let ql = i * q / b;
        let qr = (i + 1) * q / b;
        for j in ql..qr {
            let mut c = cc[xx[j]];
            if let Some(k) = (ql..j).rfind(|&k| g[xx[k]].contains(&xx[j])) {
                c = yy[k];
            }
            println!("{c}");
        }

        for j in ql..qr {
            if !xx[(j + 1)..qr].contains(&xx[j]) {
                cc[xx[j]] = yy[j];
                for k in g[xx[j]].iter().copied() {
                    cc[k] = yy[j];
                }
            }
        }
    }
}

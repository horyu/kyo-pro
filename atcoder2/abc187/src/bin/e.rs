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
        aabb: [(Usize1, Usize1); n - 1],
        q: usize,
        tteexx: [(usize, Usize1, isize); q],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    // 根0からの距離
    let mut dd = vec![!0; n];
    let mut qq = VecDeque::new();
    qq.push_back(0);
    dd[0] = 0;
    while let Some(q) = qq.pop_front() {
        for c in g[q].iter().copied() {
            if dd[c] == !0 {
                dd[c] = dd[q] + 1;
                qq.push_back(c);
            }
        }
    }
    // iから葉に向かう和
    let mut ww = vec![0; n];
    let mut rrss = vec![0; n];
    for (t, e, x) in tteexx.iter().copied() {
        let (a, b) = aabb[e];
        let (ok, ng) = if t == 1 { (a, b) } else { (b, a) };
        if dd[ok] < dd[ng] {
            ww[0] += x;
            ww[ng] -= x;
        } else {
            ww[ok] += x;
        }
    }
    // eprintln!("{dd:?}");
    // eprintln!("{ww:?}");
    let mut qq = VecDeque::new();
    qq.push_back((0, !0));
    while let Some((q, from)) = qq.pop_front() {
        rrss[q] += ww[q];
        for c in g[q].iter().copied() {
            if c != from {
                ww[c] += ww[q];
                qq.push_back((c, q));
            }
        }
    }
    let rs = rrss.iter().join("\n");
    println!("{rs}");
}

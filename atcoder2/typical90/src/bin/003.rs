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
        aabb: [(Usize1, Usize1); n - 1]
    };
    let mut g = vec![vec![]; n];
    for (u, v) in aabb.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // 木の直径を求める
    let mut start = 0;
    let mut dd = vec![!0; n];
    for i in 0..2 {
        if i == 1 {
            dd = vec![!0; n];
        }
        let mut qq = VecDeque::new();
        qq.push_back(start);
        dd[start] = 0;
        while let Some(u) = qq.pop_front() {
            for &v in g[u].iter() {
                if dd[v] == !0 {
                    dd[v] = dd[u] + 1;
                    qq.push_back(v);
                }
            }
        }
        start = dd.iter().position_max().unwrap();
    }
    let rs = dd[start] + 1;
    println!("{rs}");
}

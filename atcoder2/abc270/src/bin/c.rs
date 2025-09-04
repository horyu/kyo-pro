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
        x: Usize1,
        y: Usize1,
        uuvv: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ff = vec![!0; n];
    let mut qq = VecDeque::new();
    qq.push_back((x, !0));
    while let Some((t, f)) = qq.pop_front() {
        ff[t] = f;
        for &next in &g[t] {
            if next != f {
                qq.push_back((next, t));
            }
        }
    }
    let mut rs = VecDeque::new();
    let mut i = y;
    while i != x {
        rs.push_front(i + 1);
        i = ff[i];
    }
    rs.push_front(x + 1);
    let rs = rs.into_iter().join(" ");
    println!("{rs}");
}

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
            aabb: [(Usize1, Usize1); m],
            q: usize,
            xxkk: [(Usize1, usize); q],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in aabb.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    for (x, k) in xxkk {
        let mut hs = HashSet::new();
        let mut qq = VecDeque::new();
        hs.insert(x);
        qq.push_back((x, 0));
        let mut rs = 0;
        while let Some((qi, qd)) = qq.pop_front() {
            rs += qi + 1;
            if k == qd {
                continue;
            }
            for i in g[qi].iter().copied() {
                if hs.insert(i) {
                    qq.push_back((i, qd + 1));
                }
            }
        }
        println!("{rs}");
    }
}

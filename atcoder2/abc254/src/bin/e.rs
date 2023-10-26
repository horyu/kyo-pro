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
        aabb: [(Usize1, Usize1); m],
        q: usize,
        xxkk: [(Usize1, usize); q],
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb.iter().copied() {
        g[a].push(b);
        g[b].push(a);
    }
    for (x, k) in xxkk {
        let mut qq = VecDeque::new();
        let mut pushed = HashSet::new();
        qq.push_back((x, k));
        pushed.insert(x);
        let mut rs = 0;
        while let Some((qi, qk)) = qq.pop_front() {
            rs += qi + 1;
            if 0 == qk {
                continue;
            }
            for i in g[qi].iter().copied() {
                if pushed.insert(i) {
                    qq.push_back((i, qk - 1));
                }
            }
        }
        println!("{rs}");
    }
}

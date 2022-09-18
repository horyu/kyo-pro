#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings, map_first_last)]
use itertools::{iproduct, izip, Itertools as _};
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
    };
    let mut g = vec![vec![]; n];
    for (a, b) in aabb {
        g[a].push(b);
    }
    let mut rs = vec![0; n];
    let mut uf = UnionFind::new(n);
    let mut sub = 0;
    for i in (1..n).rev() {
        for &v in &g[i] {
            if uf.union(i, v) {
                sub += 1;
            }
        }
        // eprintln!("{i}:{}@{}; {}", n - i - sub, sub, g[i].iter().join("|"));
        rs[i - 1] = n - i - sub;
    }
    for rs in rs {
        println!("{rs}");
    }
}

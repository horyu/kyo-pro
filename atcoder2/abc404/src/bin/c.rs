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
        uuvv: [(Usize1, Usize1); m],
    };
    if n == m {
        let mut dsu = ac_library::Dsu::new(n);
        let mut g = vec![vec![]; n];
        for (u, v) in uuvv.iter().copied() {
            dsu.merge(u, v);
            g[u].push(v);
            g[v].push(u);
        }
        if dsu.size(0) == n && g.into_iter().all(|x| x.len() == 2) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

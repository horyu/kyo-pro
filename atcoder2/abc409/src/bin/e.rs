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
        mut xx: [isize; n],
        uuvvww: [(Usize1, Usize1, isize); n - 1],
    };
    let mut g = vec![HashMap::new(); n];
    for (u, v, w) in uuvvww.iter().copied() {
        g[u].insert(v, w);
        g[v].insert(u, w);
    }
    let mut ii = VecDeque::new();
    let mut pushed = vec![false; n];
    for (i, hm) in g.iter().enumerate() {
        if hm.len() == 1 {
            ii.push_back(i);
            pushed[i] = true;
        }
    }
    let mut rs = 0isize;
    while let Some(i) = ii.pop_front() {
        for (j, w) in std::mem::take(&mut g[i]) {
            rs += xx[i].abs() * w;
            xx[j] += xx[i];
            if pushed[j] {
                continue;
            }
            g[j].remove(&i);
            if g[j].len() == 1 {
                ii.push_back(j);
                pushed[j] = true;
            }
        }
    }
    println!("{rs}");
}

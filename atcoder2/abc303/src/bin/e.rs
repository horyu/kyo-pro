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
        uuvv: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![HashSet::new(); n];
    for (u, v) in uuvv.iter().copied() {
        g[u].insert(v);
        g[v].insert(u);
    }
    let mut rs = vec![];
    let mut ll = g.iter().positions(|vv| vv.len() == 1).collect_vec();
    let mut removed = vec![false; n];
    while let Some(l) = ll.pop() {
        if removed[l] {
            continue;
        }
        let center = g[l].iter().next().copied().unwrap();
        rs.push(g[center].len());
        let mut jj = vec![];
        for i in g[center].iter().copied() {
            removed[i] = true;
            if 1 < g[i].len() {
                jj.push(i);
            }
        }
        for j in jj {
            let kk = g[j].iter().copied().collect_vec();
            for k in kk {
                g[k].remove(&j);
                if k != center && g[k].len() == 1 {
                    ll.push(k);
                }
            }
        }
    }
    rs.sort_unstable();
    println!("{}", rs.iter().join(" "));
}

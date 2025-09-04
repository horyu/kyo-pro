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
        xxyy: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    let mut roots = vec![true; n];
    for (x, y) in xxyy.iter().copied() {
        g[x].push(y);
        roots[y] = false;
    }
    let roots = roots.into_iter().positions(|tf| tf).collect_vec();
    // eprintln!("{roots:?}");
    use pathfinding::directed::topological_sort::topological_sort;
    if let Ok(vv) = topological_sort(&roots, |&i| g[i].iter().copied()) {
        if let Ok(ww) = topological_sort(&roots, |&i| g[i].iter().copied().rev()) {
            if roots.len() == 1 && vv.len() == n && vv == ww {
                println!("Yes");
                let mut rrss = vec![0; n];
                for (i, v) in vv.iter().copied().enumerate() {
                    rrss[v] = i + 1;
                }
                println!("{}", rrss.iter().join(" "));
                return;
            }
        }
    }
    println!("No");
}

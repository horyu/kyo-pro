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
        first: Chars,
        last: Chars,
        n: usize,
        mut ss: [Chars; n],
    };
    ss.push(first);
    ss.push(last);

    let len = ss.len();
    let mut g = vec![vec![]; len];
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            if izip!(&ss[i], &ss[j]).filter(|(ci, cj)| ci != cj).count() <= 1 {
                g[i].push(j);
                g[j].push(i);
            }
        }
    }
    if let Some(ii) = pathfinding::prelude::bfs(&n, |&i| g[i].iter().copied(), |&i| i == n + 1) {
        println!("{}", ii.len() - 2);
        for i in ii {
            println!("{}", ss[i].iter().join(""));
        }
    } else {
        println!("-1");
    }
}

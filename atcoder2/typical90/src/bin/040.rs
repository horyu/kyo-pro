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
        w: isize,
        aa: [isize; n],
    };
    let mut g = ac_library::maxflow::MfGraph::new(n + 2);
    let mut sum = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        g.add_edge(0, i + 1, a);
        g.add_edge(i + 1, n + 1, w);
        sum += a;
    }
    for i in 0..n {
        input! {
            k: usize,
            cc: [Usize1; k],
        }
        for c in cc {
            g.add_edge(c + 1, i + 1, 1e15 as isize);
        }
    }
    let rs = sum - g.flow(0, n + 1);
    println!("{rs}");
}

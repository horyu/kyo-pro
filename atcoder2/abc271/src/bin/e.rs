#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
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
        k: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
        ee: [Usize1; k],
    };
    let mut lens = vec![std::usize::MAX; n];
    lens[0] = 0;
    for e in ee {
        let (a, b, c) = aabbcc[e];
        if a == 0 {
            lens[b] = lens[b].min(c);
        } else if b != 0 && lens[a] < std::usize::MAX {
            lens[b] = lens[b].min(lens[a] + c);
        }
    }
    if lens[n - 1] == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", lens[n - 1]);
    }
}

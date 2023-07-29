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
        mut aa: [usize; n],
        mut bb: [usize; m],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    bb.reverse();
    let xx = chain!(aa.iter().copied(), bb.iter().copied())
        .flat_map(|x| [x - 1, x, x + 1])
        .sorted_unstable()
        .dedup()
        .collect_vec();
    for x in xx {
        let ai = aa.partition_point(|&a| a <= x);
        let bi = bb.partition_point(|&b| x <= b);
        // eprintln!("{x} {ai} {bi}");
        if bi <= ai {
            println!("{x}");
            return;
        }
    }
}

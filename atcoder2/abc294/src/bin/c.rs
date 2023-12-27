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
        aa: [usize; n],
        bb: [usize; m],
    };
    let cc = chain(&aa, &bb)
        .copied()
        .sorted_unstable()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<_, _>>();
    let rs = aa.into_iter().map(|a| cc[&a] + 1).join(" ");
    println!("{rs}");
    let rs = bb.into_iter().map(|b| cc[&b] + 1).join(" ");
    println!("{rs}");
}

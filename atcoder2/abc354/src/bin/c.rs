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
        mut aacc: [(usize, usize); n],
    };
    let ac2idx = aacc
        .iter()
        .copied()
        .enumerate()
        .map(|(i, ac)| (ac, i))
        .collect::<HashMap<_, _>>();
    aacc.sort_unstable_by_key(|&(a, c)| (c, R(a)));
    aacc.dedup_by_key(|ac| ac.1);
    let mut rrss = vec![];
    let mut pre_c = usize::MAX;
    let mut pre_a = 0;
    for (a, c) in aacc {
        if a < pre_a && pre_c < c {
            continue;
        }
        pre_a = a;
        pre_c = c;
        rrss.push(ac2idx[&(a, c)] + 1);
    }
    rrss.sort_unstable();
    println!("{}", rrss.len());
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}

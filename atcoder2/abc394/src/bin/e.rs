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
        ccc: [Chars; n],
    };
    let mut ttt = vec![vec![]; n];
    let mut fff = vec![vec![]; n];
    for (f, cc) in ccc.iter().enumerate() {
        for (t, c) in cc.iter().copied().enumerate() {
            if c == '-' {
                continue;
            }
            ttt[f].push((t, c));
            fff[t].push((f, c));
        }
    }
    let mut rrss = vec![vec![!0; n]; n];
    for from in 0..n {
        if ccc[from][from] != '-' {
            rrss[from][from] = 0;
        }
    }
    // println!("{rs}");
}

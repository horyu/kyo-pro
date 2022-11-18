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
        _w: usize,
    };
    let mut vv = Vec::with_capacity(300);
    for exp in [0, 2, 4] {
        for i in 1usize..=99 {
            vv.push(i * 10usize.pow(exp));
        }
    }
    println!("{}\n{}", vv.len(), vv.into_iter().join(" "));
}

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
    let mut bts = BTreeSet::from_iter(aa.into_iter().enumerate().map(|(i, a)| (a, i)));
    let mut rs = 0;
    for b in bb {
        if let Some(&ai) = bts.range((b, 0)..).next() {
            bts.remove(&ai);
            rs += ai.0;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{rs}");
}

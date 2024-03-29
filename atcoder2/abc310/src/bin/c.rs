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
        mut ss: [Chars; n],
    };
    let mut hs = HashSet::new();
    let mut rs = 0;
    for s in ss {
        let rev = s.iter().copied().rev().collect_vec();
        let min = s.min(rev);
        if hs.insert(min) {
            rs += 1;
        }
    }
    println!("{rs}");
}

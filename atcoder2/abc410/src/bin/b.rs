#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
        xx: [usize; q],
    };
    let mut bb = vec![0; n];
    let mut rs = vec![];
    for x in xx {
        if x == 0 {
            let pos = bb.iter().copied().position_min().unwrap();
            bb[pos] += 1;
            rs.push(pos + 1);
        } else {
            bb[x - 1] += 1;
            rs.push(x);
        }
    }
    let rs = rs.into_iter().join(" ");
    println!("{rs}");
}

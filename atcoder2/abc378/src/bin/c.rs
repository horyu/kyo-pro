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
        aa: [usize; n],
    };
    let mut hm = HashMap::new();
    let mut bb = vec![];
    for (i, a) in aa.iter().copied().enumerate() {
        if let Some(pre) = hm.insert(a, (i + 1) as isize) {
            bb.push(pre);
        } else {
            bb.push(-1);
        }
    }
    let rs = bb.iter().join(" ");
    println!("{rs}");
}

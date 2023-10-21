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
    // |i - j| = ai + aj
    // o:  i - ai = aj + j
    // x: -i - ai = aj + j
    let mut hm = HashMap::new();
    for (i, &a) in aa.iter().enumerate() {
        *hm.entry(i + a).or_insert(0) += 1;
    }
    let mut rs = 0usize;
    for (i, &a) in aa.iter().enumerate() {
        if a < i {
            if let Some(x) = hm.get(&(i - a)) {
                rs += x;
            }
        }
    }
    println!("{rs}");
}

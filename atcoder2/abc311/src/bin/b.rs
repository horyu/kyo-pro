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
        d: usize,
        ss: [Chars; n],
    };
    let mut vv = vec![];
    for i in 0..d {
        vv.push(ss.iter().all(|s| s[i] == 'o'));
    }
    let mut rs = 0;
    for (tf, vv) in vv.into_iter().group_by(|&tf| tf).into_iter() {
        if tf {
            rs = rs.max(vv.count());
        }
    }
    println!("{rs}");
}

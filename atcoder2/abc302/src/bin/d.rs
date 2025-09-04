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
        m: usize,
        d: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    let bts = BTreeSet::from_iter(bb);
    let mut rs = 0;
    for a in aa {
        if let Some(&b) = bts.range(0..=(a + d)).next_back() {
            if a.abs_diff(b) <= d {
                rs = rs.max(a + b);
            }
        }
    }
    if rs == 0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}

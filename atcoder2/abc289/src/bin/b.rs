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
        aa: [usize; m],
    };
    let mut llrr: Vec<(usize, usize)> = vec![];
    for a in aa {
        if let Some(lr) = llrr.last_mut() {
            if lr.1 + 1 == a {
                lr.1 = a;
                continue;
            }
        }
        llrr.push((a, a));
    }
    let mut rs = (1..=n).collect_vec();
    for (l, r) in llrr {
        rs[(l - 1)..=r].reverse();
    }
    let rs = rs.iter().join(" ");
    println!("{rs}");
}

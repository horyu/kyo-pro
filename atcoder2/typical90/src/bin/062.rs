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
        aabb: [(Usize1, Usize1); n],
    };
    let mut ppp = vec![HashSet::new(); n];
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        if i == a || i == b {
            pushed[i] = true;
            qq.push_back(i);
        }
        for j in [a, b] {
            if i != j {
                ppp[j].insert(i);
            }
        }
    }
    let mut rs = vec![];
    while let Some(qi) = qq.pop_front() {
        rs.push(qi + 1);
        for qj in ppp[qi].iter().copied() {
            if pushed[qj] {
                continue;
            }
            pushed[qj] = true;
            qq.push_back(qj);
        }
    }
    if rs.len() == n {
        println!("{}", rs.into_iter().rev().join("\n"));
    } else {
        println!("-1");
    }
}

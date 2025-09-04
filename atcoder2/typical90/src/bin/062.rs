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
        aabb: [(Usize1, Usize1); n],
    };
    // 依存元への辺を張る
    let mut ff = vec![vec![]; n];
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; n];
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        ff[a].push(i);
        ff[b].push(i);
        if a == i || b == i {
            pushed[i] = true;
            qq.push_back(i);
        }
    }
    let mut rrss = vec![];
    while let Some(qi) = qq.pop_front() {
        rrss.push(qi + 1);
        for i in ff[qi].iter().copied() {
            if pushed[i] {
                continue;
            }
            pushed[i] = true;
            qq.push_back(i);
        }
    }
    if rrss.len() == n {
        let rs = rrss.into_iter().rev().join("\n");
        println!("{rs}");
    } else {
        println!("-1");
    }
}

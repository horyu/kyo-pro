#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        xx: [Usize1; n],
        cc: [usize; n],
    };

    let mut ff = vec![HashSet::new(); n];
    for (i, x) in xx.iter().copied().enumerate() {
        ff[x].insert(i);
    }

    let mut checked = vec![false; n];
    let mut leaves = ff.iter().positions(|f| f.is_empty()).collect_vec();
    while !leaves.is_empty() {
        let mut new_leaves = vec![];
        for leaf in leaves {
            checked[leaf] = true;
            let x = xx[leaf];
            ff[x].remove(&leaf);
            if ff[x].is_empty() {
                new_leaves.push(x);
            }
        }
        leaves = new_leaves;
    }

    let mut rs = 0usize;
    for i in 0..n {
        if checked[i] {
            continue;
        }
        checked[i] = true;
        let mut min = cc[i];
        let mut j = xx[i];
        while j != i {
            checked[j] = true;
            min = min.min(cc[j]);
            j = xx[j];
        }
        rs += min;
    }

    println!("{rs}");
}

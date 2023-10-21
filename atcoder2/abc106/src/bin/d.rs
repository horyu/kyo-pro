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
        m: usize,
        q: usize,
        llrr: [(usize, usize); m],
        ppqq: [(usize, usize); q],
    };
    let mut vvv = vec![vec![0usize; 505]; 505];
    for (l, r) in llrr.iter().copied() {
        vvv[l][r] += 1;
    }
    for i in 1..=n {
        for j in 2..=(n + 1) {
            vvv[i][j] += vvv[i][j - 1];
        }
    }
    for (p, q) in ppqq {
        let mut rs = 0;
        for i in p..=q {
            rs += vvv[i][q];
        }
        println!("{rs}");
    }
}

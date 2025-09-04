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
        ccpp: [(Usize1, usize); n],
        q: usize,
        llrr: [(Usize1, Usize1); q],
    };
    let mut s = vec![vec![0; n + 1]; 2];
    for (i, (c, p)) in ccpp.into_iter().enumerate() {
        for j in 0..2 {
            s[j][i + 1] = s[j][i];
            if c == j {
                s[j][i + 1] += p;
            }
        }
    }
    for (l, r) in llrr {
        println!("{} {}", s[0][r + 1] - s[0][l], s[1][r + 1] - s[1][l]);
    }
}

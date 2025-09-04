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
        s: Chars,
        cc: [Usize1; n],
    };
    let mut iii = vec![vec![]; m];
    for (i, c) in cc.iter().copied().enumerate() {
        iii[c].push(i);
    }
    let mut jjj = iii.clone();
    for jj in jjj.iter_mut() {
        jj.rotate_right(1);
    }
    let mut rs = s.clone();
    for (ii, jj) in izip!(iii, jjj) {
        for (i, j) in izip!(ii, jj) {
            rs[i] = s[j];
        }
    }
    println!("{}", rs.iter().join(""));
}

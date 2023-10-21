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
        mut ccllrr: [(usize, usize, usize); m],
    };
    ccllrr.sort_unstable();

    let mut uf = UnionFind::new(n + 2);
    let mut c1 = 0;
    let mut c2 = 0;
    for (c, l, r) in ccllrr {
        if uf.union(l - 1, r) {
            c1 += c;
            c2 += 1;
        }
    }

    if c2 == n {
        println!("{c1}");
    } else {
        println!("-1");
    }
}

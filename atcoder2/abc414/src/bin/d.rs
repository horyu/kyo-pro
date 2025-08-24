#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
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
        mut xx: [usize; n],
    };
    xx.sort_unstable();
    xx.dedup();
    let n = xx.len();
    if n <= m {
        println!("0");
        return;
    }
    // 距離が近い順にまとめる
    let mut bts = BTreeSet::new();
    for (i, (xl, xr)) in xx.iter().tuple_windows().enumerate() {
        bts.insert((xr - xl, i));
    }
    let mut dsu = ac_library::Dsu::new(n);
    for (_d, i) in bts.into_iter().take(n - m) {
        dsu.merge(i, i + 1);
    }
    let mut rs = 0;
    for ii in dsu.groups() {
        let l = xx[ii[0]];
        let r = xx[ii[ii.len() - 1]];
        rs += r - l;
    }
    println!("{rs}");
}

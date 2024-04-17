#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
        k: usize,
        aaa: [[usize; n]; n],
    };
    let mut rs = ModInt998244353::new(1);

    let mut dsu = ac_library::Dsu::new(n);
    for (ix, iy) in (0..n).tuple_combinations() {
        if (0..n).all(|j| aaa[j][ix] + aaa[j][iy] <= k) {
            dsu.merge(ix, iy);
        }
    }
    for vv in dsu.groups() {
        for k in 1..=vv.len() {
            rs *= k;
        }
    }

    let mut dsu = ac_library::Dsu::new(n);
    for (jx, jy) in (0..n).tuple_combinations() {
        if (0..n).all(|i| aaa[jx][i] + aaa[jy][i] <= k) {
            dsu.merge(jx, jy);
        }
    }
    for vv in dsu.groups() {
        for k in 1..=vv.len() {
            rs *= k;
        }
    }

    println!("{rs}");
}

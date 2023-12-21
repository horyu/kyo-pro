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
    // 行
    let mut rs = ModInt998244353::new(1);
    let mut dsu = ac_library::Dsu::new(n);
    for il in 0..n {
        for ir in (il + 1)..n {
            if (0..n).all(|j| aaa[il][j] + aaa[ir][j] <= k) {
                dsu.merge(il, ir);
            }
        }
    }
    for g in dsu.groups() {
        for w in 1..=g.len() {
            rs *= w;
        }
    }
    // 列
    let mut dsu = ac_library::Dsu::new(n);
    for jl in 0..n {
        for jr in (jl + 1)..n {
            if (0..n).all(|i| aaa[i][jl] + aaa[i][jr] <= k) {
                dsu.merge(jl, jr);
            }
        }
    }
    for g in dsu.groups() {
        for w in 1..=g.len() {
            rs *= w;
        }
    }
    println!("{rs}");
}

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
        k: usize,
    };
    const MOD: usize = 998244353;
    // https://github.com/E869120/kyopro_educational_90/blob/main/sol/090-02.cpp
    let mut rs = ndarray::arr2(&[[1usize, 0], [0, 1]]);
    let mut mat = ndarray::arr2(&[[1usize, 1], [1, 0]]);
    let mut b = n;
    while 0 < b {
        if b & 1 == 1 {
            rs = rs.dot(&mat);
            rs %= MOD;
        }
        mat = mat.dot(&mat);
        mat %= MOD;
        b >>= 1;
    }
    let rs = (rs[[0, 0]] + rs[[1, 0]]) % MOD;
    println!("{rs}");
}

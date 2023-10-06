#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![allow(dead_code)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

const MOD: usize = 998244353;
fn main() {
    input! {n: usize, k: usize};
    if k == 1 {
        main12(n, k);
    } else {
        main3(n, k);
    }
}

fn main3(n: usize, k: usize) {
    assert!(n <= 6 && k <= 6);
    let rs = (0..n)
        .map(|_| 0..=k)
        .multi_cartesian_product()
        .filter(|aa| {
            for (l, al) in aa.iter().enumerate() {
                let mut min = al;
                for (r, ar) in aa.iter().enumerate().skip(l + 1) {
                    min = min.min(ar);
                    if k < min * (r - l + 1) {
                        return false;
                    }
                }
            }
            true
        })
        .count();
    println!("{rs}");
}

fn main12(n: usize, k: usize) {
    assert!(k <= 1);
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

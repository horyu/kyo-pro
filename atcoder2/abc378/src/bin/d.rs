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
        h: usize,
        w: usize,
        k: usize,
        sss: [Chars; h],
    };
    let mut ng = 0u128;
    for (i, ss) in sss.iter().enumerate() {
        for (j, s) in ss.iter().copied().enumerate() {
            if s == '#' {
                ng |= 1 << (i * w + j);
            }
        }
    }
    let mut rs = 0usize;
    for i in 0..h {
        for j in 0..w {
            rs += dfs(i, j, h, w, k, ng, 0u128);
        }
    }
    println!("{rs}");
}

fn dfs(i: usize, j: usize, h: usize, w: usize, mut k: usize, ng: u128, mut used: u128) -> usize {
    if used & (1 << (i * w + j)) != 0 {
        return 0;
    }
    if ng & (1 << (i * w + j)) != 0 {
        return 0;
    }
    if k == 0 {
        return 1;
    }
    let mut rs = 0;
    k -= 1;
    used |= 1 << (i * w + j);
    if 0 < i {
        rs += dfs(i - 1, j, h, w, k, ng, used);
    }
    if i < h - 1 {
        rs += dfs(i + 1, j, h, w, k, ng, used);
    }
    if 0 < j {
        rs += dfs(i, j - 1, h, w, k, ng, used);
    }
    if j < w - 1 {
        rs += dfs(i, j + 1, h, w, k, ng, used);
    }
    rs
}

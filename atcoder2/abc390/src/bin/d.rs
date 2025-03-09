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
        aa: [usize; n],
    };
    // https://atcoder.jp/contests/abc390/editorial/12045
    let mut ss = [0usize; 12];
    let mut hs = HashSet::new();
    dfs(&aa, &mut ss, &mut hs, 0, 0, 0);
    let rs = hs.len();
    println!("{rs}");
}

fn dfs(
    aa: &[usize],
    ss: &mut [usize; 12],
    hs: &mut HashSet<usize>,
    idx: usize,
    size: usize,
    mut val: usize,
) {
    for i in 0..=size {
        val ^= ss[i];
        ss[i] += aa[idx];
        val ^= ss[i];
        if idx == aa.len() - 1 {
            hs.insert(val);
        } else if i < size {
            dfs(aa, ss, hs, idx + 1, size, val);
        } else {
            dfs(aa, ss, hs, idx + 1, size + 1, val);
        }
        val ^= ss[i];
        ss[i] -= aa[idx];
        val ^= ss[i];
    }
}

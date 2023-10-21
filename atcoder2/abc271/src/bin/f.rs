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
        aaa: [[usize; n]; n],
    };
    // https://seelx3.hatenablog.jp/entry/2022/10/02/004652
    let mut lu = vec![HashMap::new(); n];
    let mut rd = lu.clone();

    for i in 0..(1 << (n - 1)) {
        let (mut x, mut y) = (0, 0);
        let mut p = 0;
        for j in 0..n {
            p ^= aaa[x][y];
            if 0 != i & (1 << j) {
                x += 1;
            } else {
                y += 1;
            }
        }
        *lu[x].entry(p).or_insert(0usize) += 1;
    }
    for i in 0..(1 << (n - 1)) {
        let (mut x, mut y) = (n - 1, n - 1);
        let mut q = 0;
        for j in 0..n {
            q ^= aaa[x][y];
            if 0 != i & (1 << j) {
                x = x.saturating_sub(1);
            } else {
                y = y.saturating_sub(1);
            }
        }
        *rd[x].entry(q).or_insert(0) += 1;
    }

    let mut rs = 0;
    for (i, (lu, rd)) in izip!(lu, rd).enumerate() {
        for (k, v) in lu {
            if let Some(vv) = rd.get(&(k ^ aaa[i][n - 1 - i])) {
                rs += v * vv;
            }
        }
    }
    println!("{rs}");
}

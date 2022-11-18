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
        aaa: [[usize; n]; n],
    };
    let mut btm = BTreeMap::new();
    for k in 0..n {
        for i in 0..n {
            if k < i {
                btm.entry(aaa[k][i]).or_insert_with(Vec::new).push((k, i))
            }
            for j in 0..n {
                if aaa[i][k] + aaa[k][j] < aaa[i][j] {
                    println!("-1");
                    return;
                }
            }
        }
    }
    let mut rs = 0usize;
    let mut bbb = vec![vec![(!0usize) >> 2; n]; n];
    for i in 0..n {
        bbb[i][i] = 0;
    }
    for (l, iijj) in btm {
        for (i, j) in iijj {
            if (0..n).all(|k| l < bbb[i][k] + bbb[k][j]) {
                rs += l;
                // eprintln!("{l} {i}-{j}");
            }
            bbb[i][j] = l.min(bbb[i][j]);
            bbb[j][i] = l.min(bbb[j][i]);
        }
    }
    println!("{rs}");
}

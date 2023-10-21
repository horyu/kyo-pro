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
        mut aaa: [[usize; w]; h],
    };
    let mut rs = vec![];
    for (i, aa) in aaa.iter_mut().enumerate() {
        for j in 0..(w - 1) {
            if aa[j].is_odd() {
                aa[j] -= 1;
                aa[j + 1] += 1;
                rs.push(format!("{} {} {} {}", i + 1, j + 1, i + 1, j + 2));
            }
        }
    }
    for i in 0..(h - 1) {
        if aaa[i][w - 1].is_odd() {
            aaa[i][w - 1] -= 1;
            aaa[i + 1][w - 1] += 1;
            rs.push(format!("{} {} {} {}", i + 1, w, i + 2, w));
        }
    }
    println!("{}", rs.len());
    for s in rs {
        println!("{s}");
    }
}

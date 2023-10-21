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
        mut aa: [isize; n],
    };
    aa.sort_unstable();
    let mut xx = vec![];
    let mut yy = vec![];
    for a in aa {
        if a.is_even() {
            xx.push(a);
        } else {
            yy.push(a);
        }
    }
    let mut rs = -1;
    let xlen = xx.len();
    if 2 <= xlen {
        rs = rs.max(xx[xlen - 2] + xx[xlen - 1]);
    }
    let ylen = yy.len();
    if 2 <= ylen {
        rs = rs.max(yy[ylen - 2] + yy[ylen - 1]);
    }
    println!("{rs}");
}

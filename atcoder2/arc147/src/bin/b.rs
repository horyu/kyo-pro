#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        mut pp: [Usize1; n],
    };
    let mut rs = vec![];
    let mut xx = vec![];
    let mut yy = vec![];
    for (i, p) in pp.iter().copied().enumerate() {
        if i & 1 != p & 1 {
            if i & 1 == 0 {
                xx.push(i);
            } else {
                yy.push(i);
            }
        }
    }
    for (x, y) in izip!(xx, yy) {
        let l = x.min(y);
        let mut r = x.max(y);
        while l + 1 != r {
            pp.swap(r - 2, r);
            rs.push(('B', r - 1));
            r -= 2;
        }
        pp.swap(l, r);
        rs.push(('A', r));
    }
    loop {
        let mut ok = true;
        for i in 0..(n - 2) {
            if pp[i + 2] < pp[i] {
                pp.swap(i, i + 2);
                rs.push(('B', i + 1));
                ok = false;
            }
        }
        if ok {
            break;
        }
    }
    // eprintln!("{}", pp.iter().join(" "));
    println!("{}", rs.len());
    for (o, i) in rs {
        println!("{o} {i}");
    }
}

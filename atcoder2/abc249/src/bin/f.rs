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
        k: usize,
        ttyy: [(usize, isize); n],
    };
    // https://atcoder.jp/contests/abc249/editorial/3789
    let mut vv = vec![(0, vec![])];
    for (t, y) in ttyy {
        if t == 1 {
            vv.push((y, vec![]));
        } else {
            vv.last_mut().unwrap().1.push(y);
        }
    }
    let mut rs = isize::MIN;
    let mut bh = BinaryHeap::new();
    let mut sum = 0;
    for (i, (s, aa)) in vv.into_iter().rev().take(k + 1).enumerate() {
        for a in aa {
            if a < 0 {
                bh.push(a);
            } else {
                sum += a;
            }
        }
        while k < i + bh.len() {
            let a = bh.pop().unwrap();
            sum += a;
        }
        rs = rs.max(s + sum);
    }
    println!("{rs}");
}

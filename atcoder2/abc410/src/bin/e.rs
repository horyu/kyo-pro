#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        aabb: [(usize, usize); n],
    };
    let mut btm = BTreeMap::new();
    btm.insert(0, 0);
    for (qi, (a, b)) in aabb.into_iter().enumerate() {
        let mut x2y = BTreeMap::new();
        let mut y2x = x2y.clone();
        for (x, y) in btm {
            let (xx, yy) = (x + a, y + b);
            if xx <= h {
                x2y.insert(xx, y);
                y2x.insert(y, xx);
            }
            if yy <= m {
                // 右上に点があれば削除
                while let Some((&ty, &tx)) = y2x.range(yy..).next_back()
                    && x <= tx
                {
                    y2x.remove(&ty);
                    x2y.remove(&tx);
                }
                if let Some((&tx, &ty)) = x2y.range(x..).next()
                    && ty <= yy
                {
                    // nop
                } else {
                    x2y.insert(x, yy);
                    y2x.insert(yy, x);
                }
            }
        }
        if x2y.is_empty() {
            println!("{qi}");
            return;
        }
        btm = x2y;
    }
    println!("{n}");
}
